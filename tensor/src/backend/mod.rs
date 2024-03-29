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

use crate::shape::Rank;
use crate::tensor::TensorBase;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TensorType<T> {
    Scalar(T),
    Tensor(TensorBase<T>),
}

impl<T> TensorType<T> {
    pub fn scalar(scalar: T) -> Self {
        Self::Scalar(scalar)
    }

    pub fn tensor(tensor: TensorBase<T>) -> Self {
        Self::Tensor(tensor)
    }

    pub fn is_scalar(&self) -> bool {
        match self {
            Self::Scalar(_) => true,
            _ => false,
        }
    }

    pub fn rank(&self) -> Rank {
        match self {
            Self::Scalar(_) => Rank::scalar(),
            Self::Tensor(tensor) => tensor.rank(),
        }
    }
}

impl<T> From<TensorBase<T>> for TensorType<T>
where
    T: Clone,
{
    fn from(tensor: TensorBase<T>) -> Self {
        if tensor.rank().is_scalar() {
            Self::Scalar(tensor.data()[0].clone())
        } else {
            Self::Tensor(tensor)
        }
    }
}

pub trait Backend {}

pub trait BackendStorage {
    type Backend: Backend;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_type() {
        let shape = (2, 3);
        let tensor = TensorBase::<f64>::ones(shape);
        let item = TensorType::tensor(tensor);

        assert_eq!(item.rank(), Rank::from(2));
    }
}
