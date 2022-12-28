/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use clap::Args;
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};

#[derive(Args, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Services {
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    up: bool,
}

impl Services {
    pub fn new(up: bool) -> Self {
        Self { up }
    }
    pub async fn handler(&self) -> AsyncResult<&Self> {
        tracing::debug!("System processing...");
        let bot = crate::services::TelegramBot::from_env(None);
        let abot = std::sync::Arc::new(bot);
        if self.up {
            tokio::spawn( async move {
                abot.spawn().await.expect("Failed to spawn the telegram bot...");
            });
        }
        Ok(self)
    }
}
