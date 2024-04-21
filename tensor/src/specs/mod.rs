/*
    Appellation: specs <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{moves::*, ndtensor::*, scalar::*};

pub(crate) mod moves;
pub(crate) mod ndtensor;
pub(crate) mod scalar;

pub(crate) mod prelude {
    pub use super::moves::*;
    pub use super::ndtensor::*;
    pub use super::scalar::*;
}

#[cfg(test)]
mod tests {}
