/*
    Appellation: oai <module>
    Contrib: FL03 <j3mccain@gmail.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use super::DEFAULT_OPENAI_ENV;

use async_openai::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, Eq, PartialEq, Serialize)]
pub struct OpenAI(String);

impl OpenAI {
    pub fn new(secret_key: Option<String>) -> Self {
        let secret_key = secret_key.unwrap_or_default();
        Self(secret_key)
    }
    pub fn from_env(secret_key: Option<&str>) -> Self {
        let secret_key = match std::env::var(secret_key.unwrap_or(DEFAULT_OPENAI_ENV)) {
            Err(_) => None,
            Ok(v) => Some(v),
        };
        Self::new(secret_key)
    }
    pub fn is_auth(&self) -> bool {
        self.0.len() > 1 && self.0.starts_with("sk-")
    }
    pub fn client(&self) -> Client {
        Client::new().with_api_key(self.0.as_str())
    }
}

impl Default for OpenAI {
    fn default() -> Self {
        Self::from_env(None)
    }
}
