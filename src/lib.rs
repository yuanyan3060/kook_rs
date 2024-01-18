mod api;
mod error;
mod kook;
mod url;

pub use api::event::Event;
pub use error::KookError;
pub use kook::Bot;
pub use kook::Kook;
pub use kook::KookHandle;
pub use kook::Token;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tracing::metadata::LevelFilter;
    use tracing_subscriber::prelude::*;

    #[tokio::test]
    async fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().with_filter(tracing_subscriber::filter::targets::Targets::new().with_default(LevelFilter::DEBUG)))
            .init();
        #[derive(Clone)]
        pub struct EchoHandle;

        impl KookHandle for EchoHandle {
            type Err = KookError;
            fn on_event(&self, kook: Arc<Kook<Self>>, event: Arc<Event>) -> impl std::future::Future<Output = Result<(), Self::Err>> + Send {
                async move {
                    tracing::info!("enter echo");
                    let Event::KMarkdown(ref text) = *event else {
                        return Ok(());
                    };
                    tracing::info!("{:#?}", kook.bot.message_create(&text.target_id, &text.content).await?);
                    Ok(())
                }
            }
        }

        let token = kook::Token::Bot("your bot token".to_string());
        let kook = Kook::new(token, EchoHandle).await?.to_arc();
        kook.event_loop().await?;
        Ok(())
    }
}
