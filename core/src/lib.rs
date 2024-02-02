/*
    Appellation: acme-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme-core
pub use self::primitives::*;

pub(crate) mod primitives;
pub(crate) mod specs;
pub(crate) mod utils;

pub mod errors;
pub mod ops;
pub mod stores;

pub mod prelude {
    pub use crate::primitives::*;
    // pub use crate::specs::*;
    // pub use crate::utils::*;

    pub use crate::ops::*;
    pub use crate::stores::*;
}
