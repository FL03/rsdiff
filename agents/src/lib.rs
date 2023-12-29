/*
    Appellation: agents <lib>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # agents
//!
pub use self::agent::*;

pub(crate) mod agent;

pub mod actors;
pub mod env;
pub mod specs;

pub mod prelude {

    pub use crate::actors::*;
    pub use crate::agent::*;
    pub use crate::env::*;
    pub use crate::specs::*;
}
