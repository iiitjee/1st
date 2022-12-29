/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use clap::Args;
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};

#[derive(Args, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Services {
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    telegram: bool,
}

impl Services {
    pub fn new(telegram: bool) -> Self {
        Self { telegram }
    }
    pub async fn handler(&self) -> AsyncResult<&Self> {
        tracing::debug!("System processing...");

        if self.telegram {
            std::process::Command::new("cargo")
                .current_dir(scsys::project_root())
                .args(["run", "-p", "pzzld-bot"])
                .status()?;
        }
        Ok(self)
    }
}
