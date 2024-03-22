/*
    Appellation: acme-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Core
//!
//!

pub use self::{primitives::*, utils::*};

pub(crate) mod primitives;
pub(crate) mod utils;

pub mod errors;
pub mod eval;
pub mod id;
pub mod ops;
pub mod specs;
pub mod stores;
pub mod types;

pub mod prelude {
    pub use crate::primitives::*;
    pub use crate::utils::*;

    pub use crate::errors::*;
    pub use crate::eval::*;
    pub use crate::id::*;
    pub use crate::ops::*;
    pub use crate::specs::prelude::*;
    pub use crate::stores::*;
    pub use crate::types::*;
}
