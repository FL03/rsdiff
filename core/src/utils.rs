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
