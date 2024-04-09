/*
    Appellation: iter <impls>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Axis, Scalar};
use crate::tensor::TensorBase;

impl<T> TensorBase<T>
where
    T: Scalar,
{
    pub fn product(&self) -> T {
        self.data().iter().copied().product()
    }
    #[doc(hidden)]
    pub fn product_axis(&self, _axis: Axis) -> T {
        unimplemented!("product_axis")
    }

    pub fn sum(&self) -> T {
        self.data().iter().copied().sum()
    }
    #[doc(hidden)]
    pub fn sum_axis(&self, _axis: Axis) -> T {
        unimplemented!("sum_axis")
    }
}

impl<T> FromIterator<T> for TensorBase<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from_vec(Vec::from_iter(iter))
    }
}
