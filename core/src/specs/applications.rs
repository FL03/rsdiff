/*
    Appellation: application <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::AsyncSpawable;
use scsys::{prelude::Logger, BoxResult};

/// Implements the base interface for creating compatible platform applications
pub trait ApplicationSpec {
    type Cnf: Clone;
    type Ctx;
    type Msg;

    fn context(&self) -> &Self::Ctx;
    fn messages(&self) -> &Vec<Self::Msg>;
    fn settings(&self) -> &Self::Cnf;
}

/// Extends the core interface to include logging capabilities
pub trait ApplicationLoggerSpec: ApplicationSpec {
    /// Creates a service handle for toggling the tracing systems implemented
    fn with_tracing(&self, level: Option<&str>) -> BoxResult<&Self> {
        // TODO: Introduce a more refined system of tracing logged events
        let mut logger = Logger::new(level.unwrap_or("info").to_string());
        logger.setup(None);
        tracing_subscriber::fmt::init();

        tracing::info!("Successfully initiated the tracing protocol...");
        Ok(self)
    }
}

#[async_trait::async_trait]
pub trait AsyncApplicationSpawner: AsyncSpawable {
    /// Signals a graceful shutdown using tokio channels
    async fn graceful_shutdown(&self) {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to terminate the runtime...");
        tracing::info!("Terminating the application and connected services...");
    }
}

pub trait CommandLineInterface {}