/*
    Appellation: tensors <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::shape::Rank;
use crate::tensor::TensorBase;
use strum::{Display, EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(Clone, Debug, EnumCount, EnumDiscriminants, EnumIs, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase"),
    strum_discriminants(derive(serde::Deserialize, serde::Serialize))
)]
#[repr(C)]
#[strum(serialize_all = "lowercase")]
#[strum_discriminants(
    derive(
        Display,
        EnumCount,
        EnumIs,
        EnumIter,
        EnumString,
        Hash,
        Ord,
        PartialOrd,
        VariantNames
    ),
    name(TensorType)
)]
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
            Self::Scalar(unsafe { tensor.into_scalar() })
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
