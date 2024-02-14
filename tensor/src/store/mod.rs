/*
    Appellation: store <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{layout::*, storage::*};

pub(crate) mod layout;
pub(crate) mod storage;

use std::sync::{Arc, RwLock};

pub type ArcTensor<T> = Arc<RwLock<Vec<T>>>;

#[cfg(test)]
mod tests {}
