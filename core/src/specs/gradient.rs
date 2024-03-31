/*
    Appellation: gradient <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait IsDifferentiable {
    /// Returns true if the function is differentiable.
    fn differentiable(&self) -> bool;
}

pub trait Gradient<T> {
    type Gradient;

    fn grad(&self, args: T) -> Self::Gradient;
}

pub trait Grad {
    type Output;

    fn grad(&self) -> Self::Output;
}

pub trait Parameter {
    type Key;
    type Value;

    fn key(&self) -> &Self::Key;

    fn value(&self) -> &Self::Value;
}
