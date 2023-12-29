/*
    Appellation: acme <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme
//!
//! Acme is a complete framework for building intelligent agents in Rust
#[cfg(feature = "agents")]
pub use acme_agents as agents;
#[cfg(feature = "core")]
pub use acme_core as core;
#[cfg(feature = "derive")]
pub use acme_derive::*;
#[cfg(feature = "macros")]
pub use acme_macros::*;

pub mod prelude {
    #[cfg(feature = "agents")]
    pub use crate::agents::prelude::*;
    #[cfg(feature = "core")]
    pub use crate::core::prelude::*;
    #[cfg(feature = "derive")]
    pub use acme_derive::*;
    #[cfg(feature = "macros")]
    pub use acme_macros::*;
}
