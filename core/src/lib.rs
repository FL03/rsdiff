/*
    Appellation: rsdiff-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Core
//!
//! The core module provides the fundamental building blocks for the rsdiff library.
//! One of the primary focuses of the library is to provide a set of primitives and utilities
//! for interpreting various operations. The core module is the foundation for the rest of the
//! library, and it is designed to be as lightweight as possible.
//!
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use self::traits::prelude::*;
// pub use self::utils::*;

#[macro_use]
pub(crate) mod macros;
#[macro_use]
pub(crate) mod seal;
#[macro_use]
pub(crate) mod utils;

pub mod error;
pub mod id;
#[macro_use]
pub mod ops;
pub mod stores;
pub mod traits;
pub mod types;

#[doc(hidden)]
pub mod exp {
    //! # Experimental
    pub mod operator;
}

#[doc(hidden)]
pub mod prelude {
    pub use crate::error::*;
    pub use crate::id::*;
    pub use crate::nested;
    pub use crate::ops::prelude::*;
    pub use crate::stores::prelude::*;
    pub use crate::traits::prelude::*;
    pub use crate::types::*;
}
