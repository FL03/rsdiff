/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use num::Float;

pub fn sigmoid<T>(x: T) -> T
where
    T: Float,
{
    (T::one() + x.neg().exp()).recip()
}

pub trait Sigmoid {
    fn sigmoid(self) -> Self;
}

impl<T> Sigmoid for T
where
    T: Float,
{
    fn sigmoid(self) -> Self {
        (T::one() + self.neg().exp()).recip()
    }
}
