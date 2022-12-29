/*
    Appellation: services <args>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::services::telegram::{TelegramBot, TelegramBotConfig};

use clap::Args;
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::task::JoinHandle;

#[derive(Args, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Services {
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    extras: bool,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    telegram: bool,
}

impl Services {
    pub fn new(extras: bool, telegram: bool) -> Self {
        Self { extras, telegram }
    }
    pub async fn handler(&self) -> AsyncResult<&Self> {
        tracing::debug!("Processing service related command...");
        if self.telegram {
            tracing::info!("Initializing the telegram bot: Puzzled (@pzzldbot)");
            let cnf = TelegramBotConfig::default();
            TelegramBot::new(cnf.clone()).spawn().await?;
            
        }
        Ok(self)
    }
}
