/*
    Appellation: telegram <module>
    Contrib: FL03 <j3mccain@gmail.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use super::{handler, Command, TelegramBotSpec, DEFAULT_ENV_KEY};

use scsys::prelude::{AsyncResult, Configurable};
use serde::{Deserialize, Serialize};

use teloxide::prelude::*;

use std::sync::Arc;
use tokio::task::JoinHandle;

/// Configuration parameters for the [TelegramBot]
#[derive(Clone, Debug, Deserialize, Hash, Eq, PartialEq, Serialize)]
pub struct TelegramBotConfig {
    pub name: String,
    token: String,
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

impl Default for TelegramBotConfig {
    fn default() -> Self {
        Self::from_env(None)
    }
}

/// A verbose bot intelligently servicing users leveraging OpenAi's ChatGPT for natural language processing of simple queries
/// The primary goal for the bot remains offering complete report generation utilities for given articles, topics, etc.
#[derive(Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq, Serialize)]
pub struct TelegramBot {
    pub cnf: TelegramBotConfig,
}

impl TelegramBot {
    pub fn new(cnf: TelegramBotConfig) -> Self {
        Self { cnf }
    }
    pub async fn handle(&self) -> JoinHandle<Arc<Self>> {
        let bot = Arc::new(self.clone());
        tokio::spawn(async {
            bot.spawn().await.expect("");
            bot
        })
    }
    pub async fn spawn(&self) -> AsyncResult<&Self> {
        Command::repl(self.bot(), handler).await;
        Ok(self)
    }
}

impl TelegramBotSpec for TelegramBot {
    fn name(&self) -> String
    where
        Self: Sized,
    {
        self.cnf.name.clone()
    }

    fn username(&self) -> String
    where
        Self: Sized,
    {
        self.cnf.username.clone()
    }

    fn bot(&self) -> Bot
    where
        Self: Sized,
    {
        Bot::new(self.cnf.token.clone())
    }
}

impl Configurable for TelegramBot {
    type Settings = TelegramBotConfig;

    fn settings(&self) -> &Self::Settings {
        &self.cnf
    }
}
