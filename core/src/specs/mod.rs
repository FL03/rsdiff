/*
    Appellation: specs <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub use self::{gradient::*, operand::*};

pub(crate) mod gradient;
pub(crate) mod operand;

pub mod func;

pub(crate) mod prelude {
    pub use super::func::*;
    pub use super::gradient::*;
    pub use super::operand::Operand;
}

#[cfg(test)]
mod tests {}
