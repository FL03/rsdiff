/*
    Appellation: linalg <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Linear Algebra
//!
//!
pub use self::{specs::*, uplo::UPLO};

pub(crate) mod specs;
pub(crate) mod uplo;

pub mod solve;
pub mod tri;

pub(crate) mod prelude {
    pub use super::specs::*;
    pub use super::uplo::UPLO;
}

#[cfg(test)]
mod tests {}
