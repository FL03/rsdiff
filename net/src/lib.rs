/*
    Appellation: acme-clusters <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Clustering desribes the manner in-which the system recoginizes nodes registered to the owner as well as the resulting tonnetz inspired surface it forms.

*/
pub mod middleware;
pub mod proxies;
pub mod servers;
pub mod signals;

use scsys::prelude::{AsyncResult, Contextual};
use servers::ServerSpec;

#[async_trait::async_trait]
pub trait WebBackend {
    type Ctx: Contextual;
    type Server: ServerSpec + Send + Sync;

    async fn client(&self) -> axum::Router;
    fn context(&self) -> Self::Ctx;
    fn server(&self) -> Self::Server;
    /// Quickstart the server with the outlined client
    async fn serve(&self) -> AsyncResult {
        self.server().serve(self.client().await).await
    }
}
