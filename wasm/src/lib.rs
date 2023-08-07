/*
    Appellation: acme <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Acme was inspired by projects like Python's FastAPI, seeking to simplify the creation of powerful Rust-native applications targeting WebAssembly runtime's.
        Additionally, Acme services the ecosystem by forming the basis of our composable runtime environment facilitated by the tandem between Proton, Flow, and Reaction.
*/
#[cfg(feature = "core")]
pub use acme_core::*;
#[cfg(feature = "derive")]
pub use acme_derive::*;
#[cfg(feature = "macros")]
pub use acme_macros::*;
#[cfg(feature = "net")]
pub use acme_net as net;

pub mod prelude {
    pub use super::*;

    #[cfg(feature = "net")]
    pub use super::net::*;
    #[cfg(feature = "core")]
    pub use super::{events::*, handlers::*, sessions::*};
}
