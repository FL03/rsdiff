/*
   Appellation: api <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::interface::*;

pub(crate) mod interface;

pub mod routes;

use std::sync::Arc;

pub fn new() -> Api {
    Api::default()
}

pub fn from_context(ctx: crate::Context) -> Api {
    Api::from(ctx)
}

pub async fn handle(ctx: Arc<crate::Context>) -> tokio::task::JoinHandle<Api> {
    tokio::spawn(async move {
        let api = Arc::new(from_context(ctx.as_ref().clone()));
        api.start().await.expect("");
        api.as_ref().clone()
    })
}
