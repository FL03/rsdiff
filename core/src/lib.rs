/*
    Appellation: acme-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{primitives::*, specs::*, utils::*};

pub mod events;
pub mod handlers;
pub mod sessions;

pub(crate) mod primitives;
pub(crate) mod utils;

pub(crate) mod specs;

pub use crate::events::specs::*;
pub use crate::handlers::specs::*;

use async_trait::async_trait;
use scsys::prelude::{AsyncResult, Logger, Result};

///
#[async_trait]
pub trait AsyncSpawnable {
    async fn spawn(&mut self) -> AsyncResult<&Self>;
}
///
pub trait BaseApplication: BaseObject + Versionable {
    fn application(&self) -> &Self {
        self
    }
    fn namespace(&self) -> String;
}

///
pub trait BaseObject {
    fn count(&self) -> usize;
    fn name(&self) -> String;
    fn slug(&self) -> String {
        self.name().to_ascii_lowercase()
    }
    fn symbol(&self) -> String;
}
///
pub trait Spawnable {
    fn spawn(&mut self) -> Result<&Self>;
}
///
pub trait Traceable {
    fn with_tracing(&self, level: Option<&str>) -> AsyncResult<&Self> {
        // TODO: Introduce a more refined system of tracing logged events
        let mut logger = Logger::new(level.unwrap_or("info").to_string());
        logger.setup(None);
        tracing_subscriber::fmt::init();
        Ok(self)
    }
}
///
pub trait Versionable {
    type Error;

    fn update(&mut self) -> Result<()>;
    fn version(&self) -> String;
}
