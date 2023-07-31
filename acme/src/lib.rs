/*
    Appellation: acme <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme
//! 
//! Acme is a complete framework for building intelligent agents in Rust

#[cfg(feature = "core")]
pub use acme_core as core;
#[cfg(feature = "derive")]
pub use acme_derive::*;
#[cfg(feature = "macros")]
pub use acme_macros::*;


pub mod prelude {
    #[cfg(feature = "core")]
    pub use super::core::prelude::*;
    #[cfg(feature = "derive")]
    pub use acme_derive::*;
    #[cfg(feature = "macros")]
    pub use acme_macros::*;
}
