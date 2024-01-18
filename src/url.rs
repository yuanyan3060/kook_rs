pub(crate) mod http_api {
    macro_rules! concat_url {
        ($e:expr) => {
            concat!("https://www.kookapp.cn", $e)
        };
    }

    pub static GUILD_LIST: &'static str = concat_url!("/api/v3/guild/list");
    pub static GUILD_VIEW: &'static str = concat_url!("/api/v3/guild/view");
    pub static GUILD_USER_LIST: &'static str = concat_url!("/api/v3/guild/user-list");
    pub static GUILD_NICKNAME: &'static str = concat_url!("/api/v3/guild/nickname");
    pub static GUILD_LEAVE: &'static str = concat_url!("/api/v3/guild/leave");

    pub static GATEWAY_INDEX: &'static str = concat_url!("/api/v3/gateway/index");
    
    pub static USER_ME: &'static str = concat_url!("/api/v3/user/me");
    pub static USER_VIEW: &'static str = concat_url!("/api/v3/user/view");
    pub static USER_OFFLINE: &'static str = concat_url!("/api/v3/user/offline");

    pub static MESSAGE_CREATE: &'static str = concat_url!("/api/v3/message/create");
}