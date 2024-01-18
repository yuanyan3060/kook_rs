use crate::{api::response::Page, error::KookResult, url::http_api};
use reqwest::header::AUTHORIZATION;
use serde::{de::DeserializeOwned, Serialize};

use super::{
    objects::Guild,
    request,
    response::{self, ResponseWrap},
};

impl crate::Bot {
    async fn http_get<T: DeserializeOwned>(&self, url: &str, query: &[(&str, &str)]) -> KookResult<T> {
        let ret = self
            .http_client
            .get(url)
            .header(AUTHORIZATION, self.token_str.as_str())
            .query(query)
            .send()
            .await?
            .text()
            .await?;
        tracing::debug!("{}", ret);
        let ret: ResponseWrap<T> = serde_json::from_str(&ret)?;
        ret.into_result()
    }

    async fn http_get_page_all<T: DeserializeOwned>(&self, url: &str, query: &[(&str, &str)]) -> KookResult<Vec<T>> {
        let mut ret = Vec::new();
        for i in 0..Page::<T>::MAX_PAGE_SIZE {
            let resp = self
                .http_client
                .get(url)
                .header(AUTHORIZATION, self.token_str.as_str())
                .query(query)
                .query(&[("page", i.to_string().as_str()), ("page_size", "50"), ("sort", "0")])
                .send()
                .await?
                .text()
                .await?;
            tracing::debug!("{}", resp);
            let resp: ResponseWrap<Page<T>> = serde_json::from_str(&resp)?;
            let resp = resp.into_result()?;
            ret.extend(resp.items);
            if resp.meta.page == resp.meta.page_total {
                break;
            }
        }
        Ok(ret)
    }

    async fn http_post<T: DeserializeOwned>(&self, url: &str, req: &impl Serialize) -> KookResult<T> {
        let ret = self
            .http_client
            .post(url)
            .header(AUTHORIZATION, self.token_str.as_str())
            .json(req)
            .send()
            .await?
            .text()
            .await?;
        tracing::debug!("{}", ret);
        let ret: ResponseWrap<T> = serde_json::from_str(&ret)?;
        ret.into_result()
    }
}

// 服务器相关列表
impl crate::Bot {
    pub async fn guild_list(&self) -> KookResult<Vec<response::GuildListItem>> {
        self.http_get_page_all(http_api::GUILD_LIST, &[]).await
    }

    pub async fn guild_view(&self, guild_id: &str) -> KookResult<Guild> {
        self.http_get(http_api::GUILD_VIEW, &[("guild_id", guild_id)]).await
    }

    pub async fn guild_user_list(&self, guild_id: &str) -> KookResult<Vec<response::GuildUserListItem>> {
        self.http_get_page_all(http_api::GUILD_USER_LIST, &[("guild_id", guild_id)]).await
    }

    pub async fn guild_nickname(&self, guild_id: &str, user_id: &str, nickname: &str) -> KookResult<()> {
        let _: response::Empty = self
            .http_post(http_api::GUILD_NICKNAME, &request::GuildNickname { guild_id, user_id, nickname })
            .await?;
        Ok(())
    }

    pub async fn guild_leave(&self, guild_id: &str) -> KookResult<Guild> {
        let ret = self.http_post(http_api::GUILD_LEAVE, &request::GuildLeave { guild_id }).await?;
        Ok(ret)
    }
}

// 获取网关连接地址
impl crate::Bot {
    pub async fn gateway_index(&self, compress: bool) -> KookResult<String> {
        let ret: response::GatewayIndex = self
            .http_get(http_api::GATEWAY_INDEX, &[("compress", (compress as u8).to_string().as_str())])
            .await?;
        Ok(ret.url)
    }
}

// 用户接口
impl crate::Bot {
    pub async fn user_me(&self) -> KookResult<response::UserMe> {
        self.http_get(http_api::USER_ME, &[]).await
    }

    pub async fn user_view(&self, user_id: &str, guild_id: impl Into<Option<&str>>) -> KookResult<response::UserView> {
        match guild_id.into() {
            None => self.http_get(http_api::USER_VIEW, &[("user_id", user_id)]).await,
            Some(guild_id) => self.http_get(http_api::USER_VIEW, &[("user_id", user_id), ("guild_id", guild_id)]).await,
        }
    }

    pub async fn user_offline(&self) -> KookResult<()> {
        let _: [u8; 0] = self.http_post(http_api::USER_OFFLINE, &()).await?;
        Ok(())
    }
}

// 频道消息接口
impl crate::Bot {
    pub async fn message_create(&self, target_id: &str, content: &str) -> KookResult<response::MessageCreate> {
        let ret = self.http_post(http_api::MESSAGE_CREATE, &request::MessageCreate{
            message_create_type: 1,
            target_id,
            content
        }).await?;
        Ok(ret)
    }
}