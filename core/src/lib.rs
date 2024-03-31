/*
    Appellation: acme-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Core
//!
//!
#[cfg(not(feature = "std"))]
extern crate alloc;

#[macro_use]
pub(crate) mod seal;

pub mod errors;
pub mod eval;
pub mod id;
pub mod math;
pub mod ops;
pub mod specs;
pub mod types;

pub mod prelude {
    pub use crate::errors::*;
    pub use crate::eval::*;
    pub use crate::id::*;
    pub use crate::ops::prelude::*;
    pub use crate::specs::prelude::*;
    pub use crate::types::*;
}
