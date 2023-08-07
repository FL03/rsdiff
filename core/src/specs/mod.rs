/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{conduit::*, event::*, objects::*};

pub(crate) mod conduit;
pub(crate) mod event;
pub(crate) mod objects;

use async_trait::async_trait;
use scsys::prelude::{AsyncResult, Logger};


///
#[async_trait]
pub trait AsyncHandle: Clone + Send + Sync {
    type Error: std::error::Error + Send + Sync;

    async fn handler(&self) -> Result<&Self, Self::Error>
    where
        Self: Sized;
}
///
pub trait Handle: Clone {
    type Error: std::error::Error + 'static;

    fn handler(&self) -> Result<&Self, Self::Error>
    where
        Self: Sized;
}


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
pub trait Spawnable {
    fn spawn(&mut self) -> scsys::Result<&Self>;
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

    fn get_previous_version(&self) -> String;
    fn update(&mut self) -> Result<(), Self::Error>;
    fn version(&self) -> String;
}