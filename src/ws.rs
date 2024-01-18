use std::sync::Arc;

use crate::error::KookResult;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
pub struct KookWebsocket {
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    shutdown_receiver: tokio::sync::watch::Receiver<()>
}

impl KookWebsocket {
    pub async fn connect(url: &str, shutdown_receiver: tokio::sync::watch::Receiver<()>) -> KookResult<Self> {
        let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
        Ok(Self { ws_stream, shutdown_receiver })
    }

    pub fn into_arc(self) -> Arc<Self> {
        Arc::new(self)
    }


}

impl crate::Kook {
    pub async fn event_loop() {
        
    }
}