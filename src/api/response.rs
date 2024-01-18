use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use crate::error::{KookError, KookResult};

use super::objects::NotifyType;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseWrap<'a, T: Deserialize<'a> = Empty> {
    code: i32,
    message: String,
    #[serde(borrow)]
    data: &'a RawValue,
    #[serde(skip)]
    mark: std::marker::PhantomData<T>,
}

impl<'a, T: Deserialize<'a>> ResponseWrap<'a, T> {
    pub(crate) fn into_result(self) -> KookResult<T> {
        match self.code {
            0 => Ok(serde_json::from_str(self.data.get())?),
            _ => Err(KookError::Api {
                code: self.code,
                message: self.message,
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Empty {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page<T> {
    pub(crate) items: Vec<T>,
    pub(crate) meta: PageMeta,
    pub(crate) sort: PageSort,
}

impl <T> Page<T> {
    pub(crate) const MAX_PAGE_SIZE: u32 = 1 << 16;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageMeta {
    pub(crate) page: i32,
    pub(crate) page_total: i32,
    pub(crate) page_size: i32,
    pub(crate) total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageSort {
    #[serde(default)]
    pub(crate) id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserMe {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "username")]
    pub username: String,

    #[serde(rename = "identify_num")]
    pub identify_num: String,

    #[serde(rename = "online")]
    pub online: bool,

    #[serde(rename = "os")]
    pub os: String,

    #[serde(rename = "status")]
    pub status: i64,

    #[serde(rename = "avatar")]
    pub avatar: String,

    #[serde(rename = "banner")]
    pub banner: String,

    #[serde(rename = "bot")]
    pub bot: bool,

    #[serde(rename = "mobile_verified")]
    pub mobile_verified: bool,

    #[serde(rename = "client_id")]
    pub client_id: String,

    #[serde(rename = "mobile_prefix")]
    pub mobile_prefix: String,

    #[serde(rename = "mobile")]
    pub mobile: String,

    #[serde(rename = "invited_count")]
    pub invited_count: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserView {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "username")]
    pub username: String,

    #[serde(rename = "identify_num")]
    pub identify_num: String,

    #[serde(rename = "online")]
    pub online: bool,

    #[serde(rename = "status")]
    pub status: i64,

    #[serde(rename = "bot")]
    pub bot: bool,

    #[serde(rename = "avatar")]
    pub avatar: String,

    #[serde(rename = "vip_avatar")]
    pub vip_avatar: String,

    #[serde(rename = "mobile_verified")]
    pub mobile_verified: bool,

    #[serde(rename = "roles")]
    pub roles: Vec<u64>,

    #[serde(rename = "joined_at")]
    pub joined_at: i64,

    #[serde(rename = "active_time")]
    pub active_time: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildListItem {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "topic")]
    pub topic: String,

    #[serde(rename = "user_id")]
    pub user_id: String,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "notify_type")]
    pub notify_type: NotifyType,

    #[serde(rename = "region")]
    pub region: String,

    #[serde(rename = "enable_open")]
    pub enable_open: bool,

    #[serde(rename = "open_id")]
    pub open_id: String,

    #[serde(rename = "default_channel_id")]
    pub default_channel_id: String,

    #[serde(rename = "welcome_channel_id")]
    pub welcome_channel_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildUserListItem {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "username")]
    pub username: String,

    #[serde(rename = "identify_num")]
    pub identify_num: String,

    #[serde(rename = "online")]
    pub online: bool,

    #[serde(rename = "status")]
    pub status: i64,

    #[serde(rename = "bot")]
    pub bot: bool,

    #[serde(rename = "avatar")]
    pub avatar: String,

    #[serde(rename = "vip_avatar")]
    pub vip_avatar: String,

    #[serde(rename = "nickname")]
    pub nickname: String,

    #[serde(rename = "roles")]
    pub roles: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayIndex {
    #[serde(rename = "url")]
    pub url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageCreate {
    #[serde(rename = "msg_id")]
    pub msg_id: String,

    #[serde(rename = "msg_timestamp")]
    pub msg_timestamp: i64,

    #[serde(rename = "nonce")]
    pub nonce: String,
}
