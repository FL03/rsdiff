/*
    Appellation: Conduit <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{channels::*, context::*, settings::*, states::*};

pub(crate) mod channels;
pub(crate) mod context;
pub(crate) mod settings;
pub(crate) mod states;

pub mod api;
pub mod cli;
pub mod runtime;

use acme::prelude::{AppSpec, AsyncSpawnable};
use scsys::prelude::{AsyncResult, Locked};
use std::{convert::From, sync::Arc};

#[tokio::main]
async fn main() -> AsyncResult {
    // Create an application instance
    let mut app = Application::default();
    // Quickstart the application runtime with the following command
    app.setup()?;
    app.spawn().await?;

    Ok(())
}

#[derive(Debug)]
pub struct Application {
    pub channels: AppChannels,
    pub ctx: Arc<Context>,
    pub runtime: Arc<runtime::Runtime>,
    pub state: Locked<State>,
}

impl Application {
    pub fn new(ctx: Arc<Context>) -> Self {
        let channels = AppChannels::new();
        let state = States::default();

        channels.ctx.0.send(ctx.as_ref().clone()).unwrap();
        channels.state.0.send(state.clone().into()).unwrap();

        Self {
            channels,
            ctx: ctx.clone(),
            runtime: Arc::new(runtime::Runtime::new(ctx)),
            state: state.into(),
        }
    }
    /// Change the application state
    pub async fn set_state(&mut self, state: States) -> AsyncResult<&Self> {
        // Update the application state
        self.state = state.clone().into();
        // Post the change of state to the according channel(s)
        self.channels.state.0.send(self.state.clone())?;
        tracing::info!("Updating the application state to {}", state);
        Ok(self)
    }
    /// Application runtime
    pub fn runtime(&self) -> &runtime::Runtime {
        self.runtime.as_ref()
    }
}

#[async_trait::async_trait]
impl AsyncSpawnable for Application {
    async fn spawn(&mut self) -> AsyncResult<&Self> {
        tracing::debug!("Spawning the application and related services...");
        self.set_state(States::Process).await?;
        // Fetch the initialized cli and process the results
        self.runtime().handler().await?;
        self.set_state(States::Complete).await?;
        self.set_state(States::Idle).await?;
        Ok(self)
    }
}

impl AppSpec<Settings> for Application {
    type Ctx = Context;

    type State = State;

    fn init() -> Self {
        Self::default()
    }

    fn context(&self) -> Self::Ctx {
        self.ctx.as_ref().clone()
    }

    fn name(&self) -> String {
        env!("CARGO_PKG_NAME").to_string()
    }

    fn settings(&self) -> Settings {
        self.ctx.settings().clone()
    }

    fn setup(&mut self) -> AsyncResult<&Self> {
        self.settings().logger().clone().setup(None);
        tracing_subscriber::fmt::init();
        tracing::debug!("Application initialized; completing setup...");
        Ok(self)
    }

    fn state(&self) -> &Locked<State> {
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
        Self::from(Context::from(data))
    }
}

impl From<Context> for Application {
    fn from(data: Context) -> Self {
        Self::new(Arc::new(data))
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.ctx.as_ref()).unwrap())
    }
}
