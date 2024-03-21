/*
    Appellation: errors <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::error::*;

pub(crate) mod error;

pub type GraphResult<T = ()> = std::result::Result<T, GraphError>;