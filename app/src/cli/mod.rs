/*
    Appellation: cli <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{args::*, commands::*, context::*};

pub(crate) mod args;
pub(crate) mod commands;

use std::sync::Arc;
use tokio::task::JoinHandle;

///
pub fn new() -> CommandLineInterface {
    CommandLineInterface::default()
}
///
pub async fn handle() -> JoinHandle<Arc<CommandLineInterface>> {
    let tmp = Arc::new(new());
    tokio::spawn(async move {
        tmp.handler().await.expect("");
        tmp
    })
}

pub(crate) mod context {
    use super::Commands;

    use clap::Parser;
    use scsys::AsyncResult;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Hash, Parser, PartialEq, Serialize)]
    #[clap(about, author, version)]
    #[clap(long_about = None)]
    pub struct CommandLineInterface {
        #[clap(subcommand)]
        pub command: Option<Commands>,
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        pub debug: bool,
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        pub update: bool,
    }

    impl CommandLineInterface {
        pub fn new() -> Self {
            Self::parse()
        }
        pub async fn handler(&self) -> AsyncResult<&Self> {
            if self.debug {}
            if let Some(cmds) = &self.command {
                cmds.handler().await?;
            }
            Ok(self)
        }
    }

    impl Default for CommandLineInterface {
        fn default() -> Self {
            Self::parse()
        }
    }
}
