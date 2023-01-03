/*
    Appellation: telegram <module>
    Contrib: FL03 <j3mccain@gmail.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use super::{handler, Command, TelegramBotConfig, TelegramBotSpec};
use acme::prelude::AsyncSpawnable;
use scsys::prelude::{AsyncResult, Configurable, Contextual};
use serde::{Deserialize, Serialize};

use teloxide::prelude::*;

use std::sync::Arc;
use tokio::task::JoinHandle;

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
    pub async fn handle(&mut self) -> JoinHandle<Arc<Self>> {
        let bot = Arc::new(self.clone());
        tokio::spawn(async move {
            bot.start().await.expect("");
            bot
        })
    }
    async fn start(&self) -> AsyncResult<&Self> {
        Command::repl(self.bot(), handler).await;
        Ok(self)
    }
}

impl TelegramBotSpec for TelegramBot {
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

impl Contextual for TelegramBot {
    type Cnf = TelegramBotConfig;
    type Ctx = Self;

    fn context(&self) -> &Self::Ctx {
        &self
    }
}

#[async_trait::async_trait]
impl AsyncSpawnable for TelegramBot {
    async fn spawn(&mut self) -> AsyncResult<&Self> {
        Command::repl(self.bot(), handler).await;
        Ok(self)
    }
}

impl std::fmt::Display for TelegramBot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
