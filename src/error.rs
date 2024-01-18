#[derive(thiserror::Error, Debug)]
pub enum KookError {
    #[error("webscocket error `{0}`")]
    Websocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("http error `{0}`")]
    Http(#[from] reqwest::Error),
    #[error("json error `{0}`")]
    Json(#[from] serde_json::Error),
    #[error("api error code:`{code}` message:`{message}`")]
    Api{
        code: i32,
        message: String
    },
    #[error("custom error:`{0}`")]
    Custom(String)
}

pub type KookResult<T> = Result<T, KookError>;
