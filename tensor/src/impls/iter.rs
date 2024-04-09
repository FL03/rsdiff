/*
    Appellation: iter <impls>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::shape::Axis;
use crate::tensor::TensorBase;
use core::iter::{Product, Sum};

impl<T> TensorBase<T>
where
    T: Copy,
{
    /// Compute the product of all elements in the tensor
    pub fn product(&self) -> T where T: Product {
        self.data().iter().copied().product()
    }
    #[doc(hidden)]
    pub fn product_axis(&self, _axis: Axis) -> T {
        unimplemented!("product_axis")
    }
    /// Compute the sum of all elements in the tensor
    pub fn sum(&self) -> T where T: Sum {
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
