/*
    Appellation: telegram <module>
    Contrib: FL03 <j3mccain@gmail.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
pub use self::{bot::*, specs::*};

pub(crate) mod bot;

use scsys::AsyncResult;
use serde::{Deserialize, Serialize};

const DEFAULT_ENV_KEY: &str = "TELOXIDE_TOKEN";

#[derive(Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq, Serialize)]
pub struct TelegramBotToken(String);

impl TelegramBotToken {
    pub fn new(token: Option<String>) -> Self {
        Self(token.unwrap_or_default())
    }
    pub fn try_from_env(&mut self, token: Option<&str>) -> AsyncResult<&Self> {
        self.0 = std::env::var(token.unwrap_or(DEFAULT_ENV_KEY))?;
        Ok(self)
    }
}

impl TryFrom<Option<&str>> for TelegramBotToken {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        let res = std::env::var(value.unwrap_or(DEFAULT_ENV_KEY))?;
        Ok(Self::new(Some(res)))
    }
}

pub(crate) mod specs {
    use teloxide::Bot;

    ///
    pub trait TelegramBotSpec {
        fn bot(&self) -> Bot
        where
            Self: Sized;
        fn name(&self) -> String
        where
            Self: Sized;
        fn username(&self) -> String
        where
            Self: Sized;
        fn bot_from_env() -> Bot
        where
            Self: Sized,
        {
            Bot::from_env()
        }
        fn bot_with_token(token: String) -> Bot
        where
            Self: Sized,
        {
            Bot::new(token)
        }
    }
}