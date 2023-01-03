/*
    Appellation: pzzldbot <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::states::*;

pub(crate) mod states;

pub mod openai;
pub mod telegram;

use acme::prelude::AsyncSpawnable;
use scsys::prelude::{AsyncResult, Logger};

///
pub type ChannelPackStd<T> = (std::sync::mpsc::Sender<T>, std::sync::mpsc::Receiver<T>);
///
pub type TokioChannelPackMPSC<T> = (tokio::sync::mpsc::Sender<T>, tokio::sync::mpsc::Receiver<T>);

#[tokio::main]
async fn main() -> AsyncResult {
    // Setup the logger
    Logger::default().setup(None);
    // Initializing the tracer
    tracing_subscriber::fmt::init();
    // Initialize and spawn the bot
    tracing::info!("Initializing the telegram bot: Puzzled (@pzzldbot)");
    telegram::TelegramBot::new(Default::default())
        .spawn()
        .await?;

    Ok(())
}
