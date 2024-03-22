/*
   Appellation: binary <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{kinds::*, operator::*};

pub(crate) mod kinds;
pub(crate) mod operator;

#[cfg(test)]
mod tests {}
