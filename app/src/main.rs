/*
    Appellation: pzzldbot <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{context::*, settings::*, states::*};

pub(crate) mod context;
pub(crate) mod settings;
pub(crate) mod states;

pub mod cli;
pub mod services;

use acme::prelude::{AppSpec, AsyncSpawnable};
use async_trait::async_trait;
use scsys::prelude::{AsyncResult, Locked, State};
use std::{
    convert::From,
    sync::{Arc, Mutex},
};

///
pub type ChannelPackStd<T> = (std::sync::mpsc::Sender<T>, std::sync::mpsc::Receiver<T>);
///
pub type TokioChannelPackMPSC<T> = (tokio::sync::mpsc::Sender<T>, tokio::sync::mpsc::Receiver<T>);

#[tokio::main]
async fn main() -> AsyncResult {
    // Create an application instance
    let mut app = Application::default();
    // Quickstart the application runtime with the following command
    app.start().await?;

    Ok(())
}

#[derive(Clone, Debug)]
pub struct Application {
    pub ctx: Context,
    pub state: Locked<State<States>>,
}

impl Application {
    pub fn new(cnf: Settings) -> Self {
        cnf.logger().clone().setup(None);
        tracing_subscriber::fmt::init();
        tracing::info!("Application initialized; completing setup...");
        let ctx = Context::from(cnf);
        let state = Arc::new(Mutex::new(Default::default()));

        Self {
            ctx,
            state,
        }
    }
    // initializes a pack of channels
    pub fn channels<T>(&self, buffer: usize) -> TokioChannelPackMPSC<T> {
        tokio::sync::mpsc::channel::<T>(buffer)
    }
    /// Change the application state
    pub async fn set_state(&mut self, state: State<States>) -> AsyncResult<&Self> {
        // Update the application state
        self.state = Arc::new(Mutex::new(state.clone()));
        // Post the change of state to the according channel(s)
        self.channels(1).0.send(self.state.clone()).await?;
        tracing::info!("Updating the application state to {}", state);
        Ok(self)
    }
    /// Application runtime
    pub async fn runtime(&mut self) -> AsyncResult {
        self.set_state(State::new(None, None, Some(States::Process)))
            .await?;
        // Fetch the initialized cli and process the results
        let cli = cli::new();
        cli.handler().await?;
        // Update the application state post-processing
        self.set_state(State::new(None, None, Some(States::Complete)))
            .await?;
        Ok(())
    }
}

#[async_trait]
impl AsyncSpawnable for Application {
    async fn spawn(&mut self) -> AsyncResult<&Self> {
        tracing::info!("Startup: Application initializing...");
        self.runtime().await?;

        Ok(self)
    }
}

impl AppSpec<Settings> for Application {

    type Ctx = Context;

    type State = State<States>;

    fn init() -> Self {
        Self::default()
    }

    fn context(&self) -> Self::Ctx {
        self.ctx.clone()
    }

    fn name(&self) -> String {
        env!("CARGO_PKG_NAME").to_string()
    }

    fn settings(&self) -> Settings {
        self.context().cnf.clone()
    }

    fn setup(&mut self) -> AsyncResult<&Self> {
        self.cnf.logger.setup(None);
        tracing_subscriber::fmt::init();
        tracing::debug!("Application initialized; completing setup...");
        Ok(self)
    }

    fn state(&self) -> &Locked<State<States>> {
        &self.state
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::from(Context::default())
    }
}

impl From<Settings> for Application {
    fn from(data: Settings) -> Self {
        Self::from(Context::from(cnf))
    }
}

impl From<Context> for Application {
    fn from(ctx: Context) -> Self {
        let state = Arc::new(Mutex::new(Default::default()));

        Self {
            ctx,
            state,
        }
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.ctx).unwrap())
    }
}
