/*
    Appellation: settings <telegram>
    Contrib: FL03 <j3mccain@gmail.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use super::DEFAULT_ENV_KEY;
use scsys::prelude::{AsyncResult, Configurable};
use serde::{Deserialize, Serialize};

use teloxide::prelude::*;

/// Configuration parameters for the [super::TelegramBot]
#[derive(Clone, Debug, Deserialize, Hash, Eq, PartialEq, Serialize)]
pub struct TelegramBotConfig {
    pub name: String,
    pub(crate) token: String,
    pub username: String,
}

impl TelegramBotConfig {
    pub fn new(name: String, token: String, username: String) -> Self {
        Self {
            name,
            token,
            username,
        }
    }
    pub fn from_env(token: Option<&str>) -> Self {
        let token = std::env::var(token.unwrap_or(DEFAULT_ENV_KEY))
            .ok()
            .unwrap();
        Self::new(Default::default(), token, Default::default())
    }
    pub fn try_from_env(token: Option<&str>) -> AsyncResult<Self> {
        let token = std::env::var(token.unwrap_or(DEFAULT_ENV_KEY))?;
        Ok(Self::new(Default::default(), token, Default::default()))
    }
}

impl Configurable for TelegramBotConfig {
    type Settings = Self;

    fn settings(&self) -> &Self::Settings {
        &self
    }
}

impl Default for TelegramBotConfig {
    fn default() -> Self {
        Self::from_env(None)
    }
}

