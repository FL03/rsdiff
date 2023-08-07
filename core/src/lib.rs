/*
    Appellation: acme-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme-core
pub use self::{primitives::*, specs::*, utils::*};

pub mod actors;
pub mod events;

pub(crate) mod primitives;
pub(crate) mod specs;
pub(crate) mod utils;

pub mod prelude {
    pub use super::actors::*;
    pub use super::events::*;
    pub use super::primitives::*;
    pub use super::specs::*;
    pub use super::utils::*;
}
