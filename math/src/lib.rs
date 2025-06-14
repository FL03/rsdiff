/*
    Appellation: acme-math <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use self::traits::prelude::*;

pub mod cluster;
pub mod linalg;
pub mod num;
pub mod props;
pub mod signal;
pub mod stats;
pub mod traits;

#[doc(hidden)]
pub mod prelude {
    pub use super::traits::prelude::*;
}
