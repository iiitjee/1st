/*
    Appellation: services <args>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::services::telegram::{TelegramBot, TelegramBotConfig, TelegramBotOperator};

use clap::Args;
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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
        tracing::info!("Setting up the workspace...");
        if self.telegram {
            let cnf = TelegramBotConfig::try_from_env(None)?;
            let bot = Arc::new(TelegramBot::new(cnf));
            tokio::spawn(async move {
                bot.spawn().await.expect("");
            });
        }
        Ok(self)
    }
}
