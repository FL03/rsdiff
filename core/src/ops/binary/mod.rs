/*
   Appellation: binary <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{kinds::*, operator::*, specs::*};

pub(crate) mod kinds;
pub(crate) mod operator;
pub(crate) mod specs;

#[cfg(test)]
mod tests {}
