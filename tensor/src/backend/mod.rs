/*
    Appellation: backend <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Backend
//!
//!
pub use self::devices::Device;

pub(crate) mod devices;

pub mod cpu;

pub enum TensorType<T> {
    Scalar(T),
    Tensor(Vec<TensorType<T>>),
}

pub trait Backend {}

pub trait BackendStorage {
    type Backend: Backend;
}

#[cfg(test)]
mod tests {}
