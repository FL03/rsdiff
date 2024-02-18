/*
    Appellation: ad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Autodifferentiation (AD)
//!
pub use self::autodiff::generate_autodiff;

pub(crate) mod autodiff;

pub mod handle;
pub mod ops;
