/*
    Appellation: tensors <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::shape::Rank;
use crate::tensor::TensorBase;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq)]
#[strum_discriminants(derive(
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    EnumString,
    Hash,
    Ord,
    PartialOrd,
    VariantNames
))]
#[strum_discriminants(name(TensorType))]
#[cfg_attr(feature = "serde", strum_discriminants(derive(Deserialize, Serialize)))]
pub enum Tensors<T> {
    Scalar(T),
    Tensor(TensorBase<T>),
}

impl<T> Tensors<T> {
    pub fn scalar(scalar: T) -> Self {
        Self::Scalar(scalar)
    }

    pub fn tensor(tensor: TensorBase<T>) -> Self {
        Self::Tensor(tensor)
    }

    pub fn rank(&self) -> Rank {
        match self {
            Self::Tensor(tensor) => tensor.rank(),
            _ => Rank::scalar(),
        }
    }
}

impl<T> From<TensorBase<T>> for Tensors<T>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_type() {
        let shape = (2, 3);
        let tensor = TensorBase::<f64>::ones(shape);
        let item = Tensors::tensor(tensor);

        assert_eq!(item.rank(), Rank::from(2));
    }
}
