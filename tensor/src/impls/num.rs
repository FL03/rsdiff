/*
    Appellation: num <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::Scalar;
use crate::tensor::TensorBase;
use num::traits::{One, Zero};

impl<T> One for TensorBase<T>
where
    T: Scalar,
{
    fn one() -> Self {
        Self::fill(1, T::one())
    }
}

impl<T> Zero for TensorBase<T>
where
    T: Scalar,
{
    fn zero() -> Self {
        Self::fill(1, T::zero())
    }

    fn is_zero(&self) -> bool {
        self.data().iter().all(|x| x.is_zero())
    }
}
