/*
    Appellation: backend <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Backend
//!
//!

pub enum TensorType<T> {
    Scalar(T),
    Tensor(Vec<TensorType<T>>),
}

#[cfg(test)]
mod tests {}
