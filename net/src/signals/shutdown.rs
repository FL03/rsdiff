/*
    Appellation: shutdown <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

/// Simple function wrapper for [tokio::signal::ctrl_c]
pub async fn shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    tracing::info!("Signal received; initiating shutdown procedures...");
}

/// An objective interface for extending structure functionality to include graceful shutdown procedures
#[async_trait::async_trait]
pub trait GracefulShutdown {
    async fn shutdown() {
        tokio::signal::ctrl_c().await.ok().unwrap()
    }
}
