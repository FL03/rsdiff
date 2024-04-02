/*
    Appellation: iter <impls>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::Scalar;
use crate::tensor::TensorBase;

impl<T> TensorBase<T> where T: Scalar {
    pub fn sum(&self) -> T {
        self.data().iter().copied().sum()
    }

    pub fn product(&self) -> T {
        self.data().iter().copied().product()
    }
}

impl<T> FromIterator<T> for TensorBase<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from_vec(Vec::from_iter(iter))
    }
}
