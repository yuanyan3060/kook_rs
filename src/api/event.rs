use std::sync::Arc;
use serde::{Deserialize, Serialize};
use super::bool_as_u8;
use super::objects::{Channel, NotifyType, Role};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Event {
    Text(TextEvent),
    Image(ImageEvent),
    Video(VideoEvent),
    File(FileEvent),
    KMarkdown(KMarkdownEvent),
    Card(CardEvent),
    Item(ItemEvent),
    System(SystemEvent),
}

impl Event {
    pub(crate) fn to_arc(self) -> Arc<Self> {
        Arc::new(self)
    }

    pub fn author_id(&self) -> &str {
        match self {
            Event::Text(e) => e.author_id.as_str(),
            Event::Image(e) => e.author_id.as_str(),
            Event::Video(e) => e.author_id.as_str(),
            Event::File(e) => e.author_id.as_str(),
            Event::KMarkdown(e) => e.author_id.as_str(),
            Event::Card(e) => e.author_id.as_str(),
            Event::Item(e) => e.author_id.as_str(),
            Event::System(e) => e.author_id.as_str(),
        }
    }
}

#[derive(Debug)]
pub struct EventType<const V: u8>;

#[derive(Debug, thiserror::Error)]
#[error("invalid event type")]
struct EventTypeError(u8);

impl<const V: u8> Serialize for EventType<V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(V)
    }
}

impl<'de, const V: u8> Deserialize<'de> for EventType<V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        if value == V {
            Ok(EventType::<V>)
        } else {
            Err(serde::de::Error::custom(EventTypeError(value)))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemEvent {
    #[serde(rename = "channel_type")]
    pub channel_type: String,

    #[serde(rename = "type")]
    event_type: EventType<255>,

    #[serde(rename = "target_id")]
    pub target_id: String,

    #[serde(rename = "author_id")]
    pub author_id: String,

    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "extra")]
    pub extra: SystemExtra,

    #[serde(rename = "msg_id")]
    pub msg_id: String,

    #[serde(rename = "msg_timestamp")]
    pub msg_timestamp: i64,

    #[serde(rename = "nonce")]
    pub nonce: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "body")]
pub enum SystemExtra {
    #[serde(rename = "added_reaction")]
    AddedReaction {
        channel_id: String,
        emoji: Emoji,
        user_id: String,
        msg_id: String,
    },
    #[serde(rename = "deleted_reaction")]
    DeletedReaction {
        channel_id: String,
        emoji: Emoji,
        user_id: String,
        msg_id: String,
    },
    #[serde(rename = "updated_message")]
    UpdatedMessage {
        channel_id: String,
        content: String,
        mention: Vec<u64>,
        mention_all: bool,
        mention_here: bool,
        mention_roles: Vec<u64>,
        updated_at: i64,
        msg_id: String,
    },
    #[serde(rename = "deleted_message")]
    DeletedMessage { channel_id: String, msg_id: String },
    #[serde(rename = "added_channel")]
    AddedChannel(Channel),
    #[serde(rename = "updated_channel")]
    UpdatedChannel(Channel),
    #[serde(rename = "deleted_channel")]
    DeletedChannel { id: String, deleted_at: i64 },
    #[serde(rename = "pinned_message")]
    PinnedMessage {
        channel_id: String,
        operator_id: String,
        msg_id: String,
    },
    #[serde(rename = "unpinned_message")]
    UnpinnedMessage {
        channel_id: String,
        operator_id: String,
        msg_id: String,
    },
    #[serde(rename = "updated_private_message")]
    UpdatedPrivateMessage {
        content: String,
        author_id: String,
        target_id: String,
        msg_id: String,
        chat_code: String,
        updated_at: i64,
    },
    #[serde(rename = "deleted_private_message")]
    DeletedPrivateMessage {
        author_id: String,
        target_id: String,
        msg_id: String,
        chat_code: String,
        deleted_at: i64,
    },
    #[serde(rename = "private_added_reaction")]
    PrivateAddedReaction {
        emoji: Emoji,
        user_id: String,
        chat_code: String,
        msg_id: String,
    },
    #[serde(rename = "private_deleted_reaction")]
    PrivateDeletedReaction {
        emoji: Emoji,
        user_id: String,
        chat_code: String,
        msg_id: String,
    },
    #[serde(rename = "joined_guild")]
    JoinedGuild { user_id: String, joined_at: i64 },
    #[serde(rename = "exited_guild")]
    ExitedGuild { user_id: String, exited_at: i64 },
    #[serde(rename = "updated_guild_member")]
    UpdatedGuildMember { user_id: String, nickname: String },
    #[serde(rename = "guild_member_online")]
    GuildMemberOnline { user_id: String, event_time: i64, guilds: Vec<String> },
    #[serde(rename = "guild_member_offline")]
    GuildMemberOffline { user_id: String, event_time: i64, guilds: Vec<String> },
    #[serde(rename = "added_role")]
    AddedRole(Role),
    #[serde(rename = "deleted_role")]
    DeletedRole(Role),
    #[serde(rename = "updated_role")]
    UpdatedRole(Role),
    #[serde(rename = "updated_guild")]
    UpdatedGuild {
        id: String,
        name: String,
        user_id: String,
        icon: String,
        notify_type: NotifyType,
        region: String,
        #[serde(with = "bool_as_u8")]
        enable_open: bool,
        open_id: i64,
        default_channel_id: String,
        welcome_channel_id: String,
    },
    #[serde(rename = "deleted_guild")]
    DeletedGuild {
        id: String,
        name: String,
        user_id: String,
        icon: String,
        notify_type: NotifyType,
        region: String,
        #[serde(with = "bool_as_u8")]
        enable_open: bool,
        open_id: i64,
        default_channel_id: String,
        welcome_channel_id: String,
    },
    #[serde(rename = "added_block_list")]
    AddedBlockList {
        operator_id: String,
        remark: String,
        user_id: Vec<String>,
    },
    #[serde(rename = "deleted_block_list")]
    DeletedBlockList { operator_id: String, user_id: Vec<String> },
    #[serde(rename = "added_emoji")]
    AddedEmoji { id: String, name: String },
    #[serde(rename = "removed_emoji")]
    RemovedEmoji { id: String, name: String },
    #[serde(rename = "updated_emoji")]
    UpdatedEmoji { id: String, name: String },
    #[serde(rename = "joined_channel")]
    JoinedChannel { user_id: String, channel_id: String, joined_at: i64 },
    #[serde(rename = "exited_channel")]
    ExitedChannel { user_id: String, channel_id: String, exited_at: i64 },
    #[serde(rename = "user_updated")]
    UserUpdated { user_id: String, username: String, avatar: String },
    #[serde(rename = "self_joined_guild")]
    SelfJoinedGuild { guild_id: String },
    #[serde(rename = "self_exited_guild")]
    SelfExitedGuild { guild_id: String },
    #[serde(rename = "message_btn_click")]
    MessageBtnClick {
        value: String,
        msg_id: String,
        user_id: String,
        target_id: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextEvent {
    #[serde(rename = "channel_type")]
    pub channel_type: String,

    #[serde(rename = "type")]
    event_type: EventType<1>,

    #[serde(rename = "target_id")]
    pub target_id: String,

    #[serde(rename = "author_id")]
    pub author_id: String,

    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "extra")]
    pub extra: TextExtra,

    #[serde(rename = "msg_id")]
    pub msg_id: String,

    #[serde(rename = "msg_timestamp")]
    pub msg_timestamp: i64,

    #[serde(rename = "nonce")]
    pub nonce: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextExtra {
    #[serde(rename = "guild_id")]
    pub guild_id: String,

    #[serde(rename = "channel_name")]
    pub channel_name: String,

    #[serde(rename = "mention")]
    pub mention: Vec<String>,

    #[serde(rename = "mention_all")]
    pub mention_all: bool,

    #[serde(rename = "mention_roles")]
    pub mention_roles: Vec<u64>,

    #[serde(rename = "mention_here")]
    pub mention_here: bool,

    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "author")]
    pub author: Author,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageEvent {
    #[serde(rename = "channel_type")]
    pub channel_type: String,

    #[serde(rename = "type")]
    event_type: EventType<2>,

    #[serde(rename = "target_id")]
    pub target_id: String,

    #[serde(rename = "author_id")]
    pub author_id: String,

    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "extra")]
    pub extra: ImageExtra,

    #[serde(rename = "msg_id")]
    pub msg_id: String,

    #[serde(rename = "msg_timestamp")]
    pub msg_timestamp: i64,

    #[serde(rename = "nonce")]
    pub nonce: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageExtra {
    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "guild_id")]
    pub guild_id: String,

    #[serde(rename = "attachments")]
    pub attachments: ImageAttachments,

    #[serde(rename = "author")]
    pub author: Author,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageAttachments {
    #[serde(rename = "type")]
    pub attachments_type: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoEvent {
    #[serde(rename = "channel_type")]
    pub channel_type: String,

    #[serde(rename = "type")]
    event_type: EventType<3>,

    #[serde(rename = "target_id")]
    pub target_id: String,

    #[serde(rename = "author_id")]
    pub author_id: String,

    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "extra")]
    pub extra: VideoExtra,

    #[serde(rename = "msg_id")]
    pub msg_id: String,

    #[serde(rename = "msg_timestamp")]
    pub msg_timestamp: i64,

    #[serde(rename = "nonce")]
    pub nonce: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoExtra {
    #[serde(rename = "guild_id")]
    pub guild_id: String,

    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "attachments")]
    pub attachments: VideoAttachments,

    #[serde(rename = "author")]
    pub author: Author,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoAttachments {
    #[serde(rename = "type")]
    pub attachments_type: String,

    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "file_type")]
    pub file_type: String,

    #[serde(rename = "size")]
    pub size: i64,

    #[serde(rename = "duration")]
    pub duration: f64,

    #[serde(rename = "width")]
    pub width: i64,

    #[serde(rename = "height")]
    pub height: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileEvent {
    #[serde(rename = "channel_type")]
    pub channel_type: String,

    #[serde(rename = "type")]
    event_type: EventType<4>,

    #[serde(rename = "target_id")]
    pub target_id: String,

    #[serde(rename = "author_id")]
    pub author_id: String,

    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "extra")]
    pub extra: FileExtra,

    #[serde(rename = "msg_id")]
    pub msg_id: String,

    #[serde(rename = "msg_timestamp")]
    pub msg_timestamp: i64,

    #[serde(rename = "nonce")]
    pub nonce: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileExtra {
    #[serde(rename = "guild_id")]
    pub guild_id: String,

    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "attachments")]
    pub attachments: FileAttachments,

    #[serde(rename = "author")]
    pub author: Author,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileAttachments {
    #[serde(rename = "type")]
    pub attachments_type: String,

    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "file_type")]
    pub file_type: String,

    #[serde(rename = "size")]
    pub size: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KMarkdownEvent {
    #[serde(rename = "channel_type")]
    pub channel_type: String,

    #[serde(rename = "type")]
    event_type: EventType<9>,

    #[serde(rename = "target_id")]
    pub target_id: String,

    #[serde(rename = "author_id")]
    pub author_id: String,

    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "extra")]
    pub extra: KMarkdownExtra,

    #[serde(rename = "msg_id")]
    pub msg_id: String,

    #[serde(rename = "msg_timestamp")]
    pub msg_timestamp: i64,

    #[serde(rename = "nonce")]
    pub nonce: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KMarkdownExtra {
    #[serde(rename = "guild_id")]
    pub guild_id: String,

    #[serde(rename = "channel_name")]
    pub channel_name: String,

    #[serde(rename = "mention")]
    pub mention: Vec<String>,

    #[serde(rename = "mention_all")]
    pub mention_all: bool,

    #[serde(rename = "mention_roles")]
    pub mention_roles: Vec<u64>,

    #[serde(rename = "mention_here")]
    pub mention_here: bool,

    #[serde(rename = "nav_channels")]
    pub nav_channels: Vec<String>,

    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "author")]
    pub author: AuthorInfo,

    #[serde(rename = "kmarkdown")]
    pub kmarkdown: Kmarkdown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorInfo {
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

    #[serde(rename = "nickname")]
    pub nickname: String,

    #[serde(rename = "roles")]
    pub roles: Vec<u64>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Kmarkdown {
    #[serde(rename = "raw_content")]
    pub raw_content: String,

    #[serde(rename = "mention_part")]
    pub mention_part: Vec<MentionPart>,

    #[serde(rename = "mention_role_part")]
    pub mention_role_part: Vec<MentionPart>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct MentionPart {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "username")]
    pub username: String,

    #[serde(rename = "full_name")]
    pub full_name: String,

    #[serde(rename = "avatar")]
    pub avatar: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardEvent {
    #[serde(rename = "channel_type")]
    pub channel_type: String,

    #[serde(rename = "type")]
    event_type: EventType<10>,

    #[serde(rename = "target_id")]
    pub target_id: String,

    #[serde(rename = "author_id")]
    pub author_id: String,

    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "extra")]
    pub extra: CardExtra,

    #[serde(rename = "msg_id")]
    pub msg_id: String,

    #[serde(rename = "msg_timestamp")]
    pub msg_timestamp: i64,

    #[serde(rename = "nonce")]
    pub nonce: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardExtra {
    #[serde(rename = "guild_id")]
    pub guild_id: String,

    #[serde(rename = "channel_name")]
    pub channel_name: String,

    #[serde(rename = "mention")]
    pub mention: Vec<String>,

    #[serde(rename = "mention_all")]
    pub mention_all: bool,

    #[serde(rename = "mention_roles")]
    pub mention_roles: Vec<u64>,

    #[serde(rename = "mention_here")]
    pub mention_here: bool,

    #[serde(rename = "nav_channels")]
    pub nav_channels: Vec<String>,

    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "author")]
    pub author: AuthorInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemEvent {
    #[serde(rename = "channel_type")]
    pub channel_type: ItemContent,

    #[serde(rename = "type")]
    event_type: EventType<12>,

    #[serde(rename = "target_id")]
    pub target_id: String,

    #[serde(rename = "author_id")]
    pub author_id: String,

    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "extra")]
    pub extra: ItemExtra,

    #[serde(rename = "msg_id")]
    pub msg_id: String,

    #[serde(rename = "msg_timestamp")]
    pub msg_timestamp: i64,

    #[serde(rename = "nonce")]
    pub nonce: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemContent {
    #[serde(rename = "type")]
    pub item_content_type: String,

    #[serde(rename = "data")]
    pub data: ItemData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemData {
    #[serde(rename = "user_id")]
    pub user_id: String,

    #[serde(rename = "target_id")]
    pub target_id: String,

    #[serde(rename = "item_id")]
    pub item_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemExtra {
    #[serde(rename = "mention")]
    pub mention: Vec<String>,

    #[serde(rename = "author")]
    pub author: AuthorInfo,

    #[serde(rename = "kmarkdown")]
    pub kmarkdown: ItemKmarkdown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemKmarkdown {
    #[serde(rename = "mention")]
    pub mention: Vec<String>,

    #[serde(rename = "mention_part")]
    pub mention_part: Vec<MentionPart>,

    #[serde(rename = "item_part")]
    pub item_part: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Emoji {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    #[serde(rename = "identify_num")]
    pub identify_num: String,

    #[serde(rename = "avatar")]
    pub avatar: String,

    #[serde(rename = "username")]
    pub username: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "nickname")]
    pub nickname: String,

    #[serde(rename = "roles")]
    pub roles: Vec<u64>,
}
