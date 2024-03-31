/*
    Appellation: arith <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Pow<T = Self> {
    type Output;

    fn pow(&self, exp: T) -> Self::Output;
}

pub trait Powc: Pow<Self::Real> {
    type Complex;
    type Real: Pow;

    fn powc(&self, exp: Self::Complex) -> Self::Output;
}

pub trait Powi<T>: Pow<T> {
    fn powi(&self, exp: T) -> Self::Output;
}

pub trait Powf<T>: Pow<T> {
    fn powf(&self, exp: T) -> Self::Output;
}
