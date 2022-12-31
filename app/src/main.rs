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

use acme::prelude::{AppSpec, AsyncSpawable};
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
    app.spawn().await?;

    Ok(())
}

#[derive(Clone, Debug)]
pub struct Application {
    pub ctx: Context,
    pub interface: Vec<Interface>,
    pub state: Locked<State<States>>,
}

impl Application {
    pub fn new(ctx: Context, state: Locked<State<States>>) -> Self {
        ctx.settings().logger().clone().setup(None);
        Self {
            ctx,
            interface: Vec::new(),
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

#[async_trait::async_trait]
impl AsyncSpawable for Application {
    async fn spawn(&mut self) -> AsyncResult<&Self> {
        self.setup()?;
        tracing::info!("Startup: Application initializing...");
        self.runtime().await?;
        Ok(self)
    }
}

impl AppSpec for Application {
    type Cnf = Settings;

    type Ctx = Context;

    type State = State<States>;

    fn init() -> Self {
        Self::default()
    }

    fn context(&self) -> Self::Ctx {
        self.ctx.clone()
    }

    fn name(&self) -> String {
        self.settings().clone().name
    }

    fn settings(&self) -> Self::Cnf {
        self.ctx.settings().clone()
    }

    fn setup(&mut self) -> AsyncResult<&Self> {
        self.settings().logger.setup(None);
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
        Self::new(Context::from(data), Default::default())
    }
}

impl From<Context> for Application {
    fn from(data: Context) -> Self {
        Self::new(data, Default::default())
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.ctx).unwrap())
    }
}

#[derive(Clone, Debug)]
pub enum Interface {
    Cli(cli::CommandLineInterface),
}

impl Default for Interface {
    fn default() -> Self {
        Self::Cli(Default::default())
    }
}
