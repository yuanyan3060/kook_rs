use std::sync::Arc;

use super::event::Event;
use crate::{
    error::{KookError, KookResult},
    kook::KookHandle,
};
use futures_util::{SinkExt, StreamExt};
use serde::{
    de::{IgnoredAny, Visitor},
    ser::SerializeStruct,
    Deserialize, Serialize,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

impl<H: KookHandle> crate::Kook<H> {
    pub async fn event_loop(self: Arc<Self>) -> KookResult<()> {
        let mut state = WsStateMachine::GetGateway;
        let mut max_sn: u64 = 0;
        loop {
            match state {
                WsStateMachine::GetGateway => match self.bot.gateway_index(false).await {
                    Ok(url) => state = WsStateMachine::ConnectGateway(url),
                    Err(err) => tracing::error!("get ws url failed: {}", err),
                },
                WsStateMachine::ConnectGateway(url) => match tokio_tungstenite::connect_async(url.as_str()).await {
                    Ok((ws_stream, _)) => state = WsStateMachine::WaitHello(ws_stream, std::time::SystemTime::now()),
                    Err(err) => {
                        tracing::error!("connect ws url failed: {}", err);
                        state = WsStateMachine::GetGateway;
                    }
                },
                WsStateMachine::WaitHello(mut ws_stream, wait_start) => match Self::next_message(&mut ws_stream).await {
                    Ok(Message::Hello { code, session_id: _ }) => {
                        if code == 0 {
                            state = WsStateMachine::Ping(ws_stream)
                        } else {
                            tracing::error!("wait hello failed err code: {}", code);
                            state = WsStateMachine::GetGateway;
                        }
                    }
                    Ok(Message::Reconnect { code, err }) => {
                        tracing::error!("reconnect code: {} err: {}", code, err);
                        state = WsStateMachine::GetGateway;
                    }
                    Ok(_) => {
                        state = if wait_start.elapsed().map(|x| x.as_secs()).unwrap_or(200) > 6 {
                            WsStateMachine::GetGateway
                        } else {
                            WsStateMachine::WaitHello(ws_stream, wait_start)
                        };
                    }
                    Err(err) => {
                        tracing::error!("wait hello failed: {}", err);
                        state = WsStateMachine::GetGateway;
                    }
                },
                WsStateMachine::Ping(mut ws_stream) => {
                    let mut interval = tokio::time::interval(std::time::Duration::from_secs(6));
                    let mut ping_count = 0;
                    loop {
                        tokio::select! {
                            _ = interval.tick() => {
                                match ping_count {
                                    0..=3 => {},
                                    4 => {
                                        match Self::ping(&mut ws_stream, max_sn).await {
                                            Ok(_) => {
                                                tracing::debug!("ping success");
                                            },
                                            Err(err) => {
                                                tracing::error!("ping failed: {}", err);
                                                state = WsStateMachine::GetGateway;
                                                break;
                                            },
                                        }
                                    },
                                    _ => {
                                        tracing::error!("ping timeout");
                                        state = WsStateMachine::GetGateway;
                                        break;
                                    }
                                }
                                ping_count += 1;
                            }
                            msg = Self::next_message(&mut ws_stream) => {
                                match msg {
                                    Ok(Message::Reconnect { code, err }) => {
                                        tracing::error!("reconnect code: {} err: {}", code, err);
                                        state = WsStateMachine::GetGateway;
                                        break;
                                    },
                                    Ok(Message::Event {sn, event}) => {
                                        max_sn = max_sn.max(sn);
                                        if self.handle.skip_self() && event.author_id() == self.bot_info.id {
                                            continue
                                        }
                                        let kook = self.clone();
                                        let event = event.to_arc();
                                        let handle = self.handle.clone();
                                        tokio::spawn(async move {
                                            match handle.on_event(kook, event).await {
                                                Ok(_) => {},
                                                Err(err) => handle.error_handle(&err),
                                            }
                                        });
                                    },
                                    Ok(Message::Pong) => {
                                        ping_count = 0;
                                    },
                                    Ok(_) => {},
                                    Err(err) => {
                                        tracing::error!("recv message failed err:{}", err);
                                    },
                                }
                            }
                        }
                    }
                }
                // WsStateMachine::Resume(_) => {} TODO
            }
        }
    }

    async fn next_message(ws_stream: &mut WsStream) -> KookResult<Message> {
        let msg = ws_stream.next().await;
        let Some(msg) = msg else {
            return Err(KookError::Custom("unreachable".to_string()));
        };
        let tokio_tungstenite::tungstenite::Message::Text(msg) = msg? else {
            return Err(KookError::Custom("ws message not match".to_string()));
        };
        tracing::debug!("recv message:{}", msg);
        let ret = serde_json::from_str(&msg)?;
        Ok(ret)
    }

    async fn ping(ws_stream: &mut WsStream, sn: u64) -> KookResult<()> {
        let body = serde_json::to_string(&Message::Ping { sn })?;
        let body = tokio_tungstenite::tungstenite::Message::Text(body);
        ws_stream.send(body).await?;
        Ok(())
    }
}

#[derive(Debug)]
enum WsStateMachine {
    GetGateway,
    ConnectGateway(String),
    WaitHello(WsStream, std::time::SystemTime),
    Ping(WsStream),
    // Resume(WsStream), TODO
}

#[derive(Debug)]
enum Message {
    Event { sn: u64, event: Event },
    Hello { code: i32, session_id: Option<String> },
    Ping { sn: u64 },
    Pong,
    Resume { sn: u64 },
    Reconnect { code: i32, err: String },
    ResumeAck { session_id: String },
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error as DeError;
        enum Field {
            S,
            D,
            Sn,
            Unknown,
        }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;
                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`s` or `d` or `sn`")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match v {
                            "s" => Ok(Field::S),
                            "d" => Ok(Field::D),
                            "sn" => Ok(Field::Sn),
                            _ => Ok(Field::Unknown),
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct TagVisitor;
        impl<'de> Visitor<'de> for TagVisitor {
            type Value = Message;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Message")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                #[derive(Deserialize)]
                #[serde(untagged)]
                enum Data {
                    Hello {
                        code: i32,
                        #[serde(default)]
                        session_id: Option<String>,
                    },
                    Reconnect {
                        code: i32,
                        err: String,
                    },
                    ResumeAck {
                        session_id: String,
                    },
                    Event(Event),
                }

                let mut s: Option<u8> = None;
                let mut d: Option<Data> = None;
                let mut sn: Option<u64> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::S => s = Some(map.next_value()?),
                        Field::D => d = Some(map.next_value()?),
                        Field::Sn => sn = Some(map.next_value()?),
                        Field::Unknown => {
                            map.next_value::<IgnoredAny>()?;
                        }
                    }
                }

                match (s.ok_or(DeError::missing_field("s"))?, d) {
                    (0, Some(Data::Event(event))) => Ok(Self::Value::Event {
                        sn: sn.ok_or(DeError::missing_field("sn"))?,
                        event,
                    }),
                    (0, _) => Err(DeError::custom("d must be Event")),
                    (1, Some(Data::Hello { code, session_id })) => Ok(Self::Value::Hello { code, session_id }),
                    (1, _) => Err(DeError::custom("d must be Hello")),
                    (2, _) => Ok(Self::Value::Ping {
                        sn: sn.ok_or(DeError::missing_field("sn"))?,
                    }),
                    (3, _) => Ok(Self::Value::Pong),
                    (4, _) => Ok(Self::Value::Resume {
                        sn: sn.ok_or(DeError::missing_field("sn"))?,
                    }),
                    (5, Some(Data::Reconnect { code, err })) => Ok(Self::Value::Reconnect { code, err }),
                    (5, _) => Err(DeError::custom("d must be Reconnect")),
                    (6, Some(Data::ResumeAck { session_id })) => Ok(Self::Value::ResumeAck { session_id }),
                    (6, _) => Err(DeError::custom("d must be ResumeAck")),
                    (num, _) => Err(DeError::unknown_variant(&num.to_string(), &["0", "1", "2", "3", "4", "5", "6"])),
                }
            }
        }
        deserializer.deserialize_map(TagVisitor)
    }
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Message::Event { sn, event } => {
                let mut s = serializer.serialize_struct("Message", 3)?;
                s.serialize_field("s", &0)?;
                s.serialize_field("d", event)?;
                s.serialize_field("sn", sn)?;
                s.end()
            }
            Message::Hello { code, session_id } => {
                let mut s = serializer.serialize_struct("Message", 2)?;
                s.serialize_field("s", &1)?;
                #[derive(Serialize)]
                struct Hello<'a> {
                    code: &'a i32,
                    session_id: &'a Option<String>,
                }
                s.serialize_field("d", &Hello { code, session_id })?;
                s.end()
            }
            Message::Ping { sn } => {
                let mut s = serializer.serialize_struct("Message", 2)?;
                s.serialize_field("s", &2)?;
                s.serialize_field("sn", sn)?;
                s.end()
            }
            Message::Pong => {
                let mut s = serializer.serialize_struct("Message", 1)?;
                s.serialize_field("s", &3)?;
                s.end()
            }
            Message::Resume { sn } => {
                let mut s = serializer.serialize_struct("Message", 2)?;
                s.serialize_field("s", &4)?;
                s.serialize_field("sn", sn)?;
                s.end()
            }
            Message::Reconnect { code, err } => {
                let mut s = serializer.serialize_struct("Message", 2)?;
                s.serialize_field("s", &5)?;
                #[derive(Serialize)]
                struct Reconnect<'a> {
                    code: &'a i32,
                    err: &'a str,
                }
                s.serialize_field("d", &Reconnect { code, err })?;
                s.end()
            }
            Message::ResumeAck { session_id } => {
                let mut s = serializer.serialize_struct("Message", 2)?;
                s.serialize_field("s", &6)?;
                #[derive(Serialize)]
                struct ResumeAck<'a> {
                    session_id: &'a str,
                }
                s.serialize_field("d", &ResumeAck { session_id })?;
                s.end()
            }
        }
    }
}
