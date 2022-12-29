/*
    Appellation: services <args>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::services::telegram::{TelegramBot, TelegramBotConfig};

use acme::prelude::AsyncSpawable;
use clap::Args;
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};

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
            TelegramBot::new(cnf).spawn().await.expect("");
        }
        Ok(self)
    }
}
