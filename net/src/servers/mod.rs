/*
    Appellation: server <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{server::*, specs::*};

pub(crate) mod server;

pub(crate) mod specs {
    use crate::signals::shutdown::shutdown;
    use hyper::server::{conn::AddrIncoming, Builder};
    use scsys::AsyncResult;
    use std::net::SocketAddr;

    pub trait ServerOperators {
        fn address(&self) -> SocketAddr {
            SocketAddr::from((self.host(), self.port()))
        }
        fn host(&self) -> [u8; 4];
        fn port(&self) -> u16;
    }

    #[async_trait::async_trait]
    pub trait ServerSpec {
        /// Fetch an instance of a [std::net::SocketAddr]
        fn address(&self) -> SocketAddr;
        /// Creates a new builder instance with the address created from the given port
        fn builder(&self) -> Builder<AddrIncoming> {
            tracing::debug!("Initializing the server");
            hyper::Server::bind(&self.address())
        }
        async fn serve(&self, client: axum::Router) -> AsyncResult {
            tracing::info!("Starting the server...");
            self.builder()
                .serve(client.into_make_service())
                .with_graceful_shutdown(shutdown())
                .await?;
            Ok(())
        }
    }
}
