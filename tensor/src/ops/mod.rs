/*
    Appellation: ops <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{backprop::*, kinds::*, op::*};

pub(crate) mod backprop;
pub(crate) mod op;

pub(crate) mod kinds {
    pub use self::reshape::*;

    pub(crate) mod reshape;
}

pub trait BaseOperation {
    type Output;

    fn name(&self) -> &str;
}

#[cfg(test)]
mod tests {}
