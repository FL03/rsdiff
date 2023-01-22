/*
   Appellation: assets <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::Context;
use axum::{
    routing::{get_service, MethodRouter},
    Extension, Router,
};
use axum_core::response::IntoResponse;
use hyper::StatusCode;
use scsys::prelude::{project_root, AsyncResult};
use tower_http::services::ServeDir;

pub fn router() -> Router {
    let path = project_root().join("examples/service");
    Router::new().nest_service(
        "/",
        get_service(ServeDir::new(path)).handle_error(handle_error),
    )
}

/// Error handler for serving static assets
async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

pub async fn asset_router(Extension(ctx): Extension<Context>) -> MethodRouter {
    let dir = format!("{}/{}", ctx.workdir().to_str().unwrap(), "dist");
    get_service(ServeDir::new(dir)).handle_error(handle_error)
}

pub struct Wasm {
    pub port: u16,
    pub workdir: String,
}

impl Wasm {
    pub fn new(port: u16, workdir: String) -> Self {
        Self { port, workdir }
    }
    pub fn address(&self) -> std::net::SocketAddr {
        std::net::SocketAddr::from(([0, 0, 0, 0], self.port))
    }
    pub async fn client(&self) -> Router {
        let serve_dir =
            get_service(ServeDir::new(self.workdir.as_str())).handle_error(handle_error);
        Router::new().nest_service("/", serve_dir)
    }
    pub async fn serve(&self) -> AsyncResult {
        axum::Server::bind(&self.address())
            .serve(self.client().await.into_make_service())
            .await
            .unwrap();
        Ok(())
    }
}

impl Default for Wasm {
    fn default() -> Self {
        let root = project_root().to_str().unwrap().to_string();
        let workdir = format!("{}/{}", root, "dist");
        Wasm::new(8080, workdir)
    }
}
