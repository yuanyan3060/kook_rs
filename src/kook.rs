use std::{
    fmt::{write, Display},
    sync::Arc, error::Error,
};

use serde::Deserialize;

use crate::{api::event::Event, error::{KookResult, KookError}};

pub struct BotInfo {
    pub id: String
}

pub struct Bot {
    pub token: Token,
    pub token_str: String,
    pub(crate) http_client: reqwest::Client,
}

pub struct Kook<H: KookHandle + Clone + 'static> {
    pub bot: Bot,
    pub bot_info: BotInfo,
    pub(crate) handle: H,
}

impl<H: KookHandle + Send + Sync + Clone> Kook<H> {
    pub async fn new(token: Token, handle: H) -> KookResult<Self> {
        let token_str = token.to_string();
        let bot = Bot {
            token,
            token_str,
            http_client: reqwest::Client::new(),
        };
        let me = bot.user_me().await?;
        Ok(Self {
            bot,
            handle,
            bot_info: BotInfo { id: me.id }
        })
    }

    pub fn to_arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

pub trait KookHandle
where
    Self: Sized + Send + Sync + Clone,
{
    type Err: Error;
    fn on_event(&self, kook: Arc<Kook<Self>>, event: Arc<Event>) -> impl std::future::Future<Output = Result<(), Self::Err>> + Send;
    fn error_handle(&self, err: &Self::Err) {
        tracing::error!("handle error:{}", err)
    }
    fn skip_self(&self) -> bool {
        true
    }
}

#[derive(Clone)]
pub struct EmptyKookHandle;

impl KookHandle for EmptyKookHandle {
    type Err = KookError;

    fn on_event(&self, _kook: Arc<Kook<Self>>, _event: Arc<Event>) -> impl std::future::Future<Output = Result<(), Self::Err>> + Send {
        async {
            Ok(())
        }
    }
}

#[derive(Deserialize)]
pub enum Token {
    Bot(String),
    Oauth2(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Bot(token) => write(f, format_args!("Bot {token}")),
            Token::Oauth2(token) => write(f, format_args!("Bearer {token}")),
        }
    }
}
