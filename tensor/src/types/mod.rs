/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{id::*, kinds::*, order::*, tensors::*};

pub(crate) mod id;
pub(crate) mod kinds;
pub(crate) mod order;
pub(crate) mod tensors;

pub(crate) mod prelude {
    pub use super::id::*;
    pub use super::kinds::*;
    pub use super::order::*;
    pub use super::tensors::*;
}
