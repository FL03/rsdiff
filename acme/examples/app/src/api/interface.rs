/*
   Appellation: api <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::{api::routes, Context};
use acme::prelude::{
    servers::{Server, ServerSpec},
    AsyncSpawnable, WebBackend,
};
use axum::Router;
use http::header::{HeaderName, AUTHORIZATION};
use scsys::AsyncResult;
use std::sync::Arc;
use tower_http::{
    compression::CompressionLayer,
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Api {
    pub ctx: Arc<Context>,
    pub server: Arc<Server>,
}

impl Api {
    pub fn new(ctx: Arc<Context>) -> Self {
        let server = Server::from(ctx.cnf.server.pieces());
        Self {
            ctx,
            server: Arc::new(server),
        }
    }
    /// Quickstart the server with the outlined client
    pub async fn start(&self) -> AsyncResult {
        self.server().serve(self.client().await).await
    }
}

impl From<Api> for Context {
    fn from(d: Api) -> Context {
        d.ctx.as_ref().clone()
    }
}

impl From<Arc<Context>> for Api {
    fn from(ctx: Arc<Context>) -> Self {
        Self::new(ctx)
    }
}

impl From<Context> for Api {
    fn from(ctx: Context) -> Self {
        Self::from(Arc::new(ctx))
    }
}

impl std::fmt::Display for Api {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ctx.settings().server().address())
    }
}

#[async_trait::async_trait]
impl AsyncSpawnable for Api {
    async fn spawn(&mut self) -> AsyncResult<&Self> {
        self.server().serve(self.client().await).await?;
        Ok(self)
    }
}

#[async_trait::async_trait]
impl WebBackend for Api {
    type Ctx = Context;

    type Server = Server;

    async fn client(&self) -> axum::Router {
        Router::new()
            .merge(routes::api())
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                    .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
            )
            .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                AUTHORIZATION,
            )))
            .layer(CompressionLayer::new())
            .layer(PropagateHeaderLayer::new(HeaderName::from_static(
                "x-request-id",
            )))
            .layer(axum::Extension(self.ctx.clone()))
    }

    fn context(&self) -> Self::Ctx {
        self.ctx.as_ref().clone()
    }

    fn server(&self) -> Self::Server {
        self.server.as_ref().clone()
    }
}
