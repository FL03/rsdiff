/*
    Appellation: specs <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub use self::operator::*;

pub(crate) mod operator;

pub mod func;
pub mod hkt;

pub(crate) mod prelude {
    pub use super::func::*;
    pub use super::hkt::*;
    pub use super::operator::*;
}

#[cfg(test)]
mod tests {}
