use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use super::bool_as_u8;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    username: String,
    nickname: String,
    identify_num: String,
    online: bool,
    bot: bool,
    status: UserStatus,
    avatar: String,
    vip_avatar: String,
    mobile_verified: bool,
    roles: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Guild {
    id: String,
    name: String,
    topic: String,
    user_id: String,
    icon: String,
    notify_type: NotifyType,
    region: String,
    #[serde(with = "bool_as_u8")]
    enable_open: bool,
    open_id: String,
    default_channel_id: String,
    welcome_channel_id: String,
    roles: Vec<Role>,
    channels: Vec<Channel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    role_id: u64,
    name: String,
    color: u64,
    position: u64,
    #[serde(with = "bool_as_u8")]
    hoist: bool,
    #[serde(with = "bool_as_u8")]
    mentionable: bool,
    permissions: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    id: String,
    name: String,
    user_id: String,
    guild_id: String,
    topic: String,
    is_category: bool,
    parent_id: String,
    level: u32,
    slow_mode: u32,
    r#type: ChannelType,
    permission_overwrites: Vec<PermissionOverwrite>,
    permission_users: Vec<PermissionUser>,
    #[serde(with = "bool_as_u8")]
    permission_sync: bool,
    has_password: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    id: String,
    r#type: i32,
    content: String,
    create_at: i64,
    author: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachments {
    r#type: String,
    url: String,
    name: String,
    size: u64,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum UserStatus {
    Normal0 = 0,
    Normal1 = 1,
    Ban = 10,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum NotifyType {
    Default = 0,
    All = 1,
    AtOnly = 2,
    None = 3,
}



#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ChannelType {
    None = 0,
    Text = 1,
    Voice = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionOverwrite {
    role_id: u64,
    allow: i32,
    deny: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionUser {
    user: User,
    allow: i32,
    deny: i32,
}
