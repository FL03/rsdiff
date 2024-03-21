/*
    Appellation: specs <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub use self::{gradient::*, operator::*};

pub(crate) mod gradient;
pub(crate) mod operator;

pub mod func;
pub mod hkt;

pub(crate) mod prelude {
    pub use super::func::*;
    pub use super::gradient::*;
    pub use super::hkt::*;
    pub use super::operator::*;
}

#[cfg(test)]
mod tests {}
