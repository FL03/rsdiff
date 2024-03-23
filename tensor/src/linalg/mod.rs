/*
    Appellation: linalg <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Linear Algebra
//!
//!
pub mod arith;
pub mod uplo;

pub trait Inverse {
    fn inverse(&self) -> Self;
}

#[cfg(test)]
mod tests {}
