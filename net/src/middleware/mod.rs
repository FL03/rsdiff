/*
    Appellation: middleware <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::specs::*;

pub(crate) mod specs {
    use tower::{Layer, Service};

    pub trait Servicable {}

    pub trait ServiceExt<Req>: Service<Req> + Servicable {}

    pub trait Middleware<S: Servicable> {}

    pub trait MiddlewareExt<S: Servicable>: Layer<S> + Middleware<S> {}
}
