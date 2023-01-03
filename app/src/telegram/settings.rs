/*
    Appellation: settings <telegram>
    Contrib: FL03 <j3mccain@gmail.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use super::DEFAULT_ENV_KEY;
use scsys::prelude::config::{Config, Environment};
use scsys::prelude::{try_collect_config_files, AsyncResult, ConfigResult, Configurable};
use serde::{Deserialize, Serialize};

/// Configuration parameters for the [super::TelegramBot]
#[derive(Clone, Debug, Deserialize, Hash, Eq, PartialEq, Serialize)]
pub struct TelegramBotConfig {
    pub name: Option<String>,
    pub(crate) token: String,
    pub username: Option<String>,
}

impl TelegramBotConfig {
    pub fn new(name: Option<String>, token: String, username: Option<String>) -> Self {
        Self {
            name,
            token,
            username,
        }
    }
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder()
            .set_default("name", env!("CARGO_PKG_NAME"))?
            .set_default("token", "")?
            .set_default("username", format!("@{}", env!("CARGO_PKG_NAME")))?
            .add_source(Environment::default().separator("__"));

        if let Ok(v) = try_collect_config_files("**/*.config.*", false) {
            builder = builder.add_source(v);
        }
        if let Ok(v) = std::env::var(DEFAULT_ENV_KEY) {
            builder = builder.set_override("token", v)?;
        };
        builder.build()?.try_deserialize()
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
        Self::build().unwrap()
    }
}
