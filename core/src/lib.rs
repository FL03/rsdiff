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
pub(crate) mod macros;
#[macro_use]
pub(crate) mod utils;

pub mod error;
pub mod id;
#[doc(hidden)]
pub mod math;
#[macro_use]
pub mod ops;
pub mod specs;
pub mod types;

pub mod prelude {
    pub use crate::error::*;
    pub use crate::id::*;
    pub use crate::nested;
    pub use crate::ops::prelude::*;
    pub use crate::specs::prelude::*;
    pub use crate::types::*;
}
