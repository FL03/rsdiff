/*
    Appellation: linalg <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Linear Algebra
//!
//!
pub use self::specs::*;

pub(crate) mod specs;

pub mod solve;
pub mod tri;
pub mod uplo;

pub(crate) mod prelude {
    pub use super::specs::*;
    pub use super::uplo::UPLO;
}

#[cfg(test)]
mod tests {}
