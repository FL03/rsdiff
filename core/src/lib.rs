/*
    Appellation: acme-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Core
//!
//!
#[cfg(not(feature = "std"))]
extern crate alloc;

// pub use self::utils::*;

#[macro_use]
pub(crate) mod seal;
#[macro_use]
pub(crate) mod utils;

pub mod error;
pub mod eval;
pub mod id;
pub mod math;
pub mod ops;
pub mod specs;
pub mod types;

pub mod prelude {
    pub use crate::error::*;
    pub use crate::eval::*;
    pub use crate::id::*;
    pub use crate::ops::prelude::*;
    pub use crate::specs::prelude::*;
    pub use crate::types::*;
}
