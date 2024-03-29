use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildNickname<'a> {
    pub(crate) guild_id: &'a str,
    pub(crate) nickname: &'a str,
    pub(crate) user_id: &'a str
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildLeave<'a> {
    pub(crate) guild_id: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct MessageCreate<'a> {
    #[serde(rename = "type")]
    pub(crate) message_create_type: i64,

    #[serde(rename = "target_id")]
    pub(crate) target_id: &'a str,

    #[serde(rename = "content")]
    pub(crate) content: &'a str,
}
