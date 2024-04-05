/*
    Appellation: arith <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Pow<T = Self> {
    type Output;

    fn pow(&self, exp: T) -> Self::Output;
}
