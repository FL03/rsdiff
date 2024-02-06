/*
    Appellation: store <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{layout::*, storage::*};

pub(crate) mod layout;
pub(crate) mod storage;

#[cfg(test)]
mod tests {}
