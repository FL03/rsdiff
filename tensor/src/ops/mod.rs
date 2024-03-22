/*
    Appellation: ops <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{backprop::*, kinds::*};

pub(crate) mod backprop;
pub(crate) mod kinds;

pub trait TensorExpr {}

#[cfg(test)]
mod tests {}
