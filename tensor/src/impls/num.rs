/*
    Appellation: num <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::Scalar;
use crate::tensor::TensorBase;
use num::traits::{Num, One, Zero};

impl<T> Num for TensorBase<T>
where
    T: Scalar + Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(Self::from_scalar)
    }
}

impl<T> One for TensorBase<T>
where
    T: Scalar,
{
    fn one() -> Self {
        Self::from_scalar(T::one())
    }
}

impl<T> Zero for TensorBase<T>
where
    T: Scalar,
{
    fn zero() -> Self {
        Self::from_scalar(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.data().iter().all(|x| x.is_zero())
    }
}
