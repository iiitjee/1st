/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::args::*;

use clap::Subcommand;
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Subcommand)]
pub enum Commands {
    Services(Services),
}

impl Commands {
    pub async fn handler(&self) -> AsyncResult<&Self> {
        tracing::info!("Processing commands issued to the cli...");
        match self {
            Self::Services(services) => {
                tracing::info!("Setting up the environment...");
                services.handler().await?;
            }
        };
        Ok(self)
    }
}

impl Default for Commands {
    fn default() -> Self {
        Self::Services(Default::default())
    }
}
