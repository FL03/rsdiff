/*
    Appellation: gradient <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Jacobian {
    type Item;

    fn jacobian(&self) -> Self::Item;
}

pub trait Partial<T> {
    type Output;

    fn partial(&self, args: T) -> Self::Output;
}

pub trait Grad {
    type Output;

    fn grad(&self) -> Self::Output;
}

pub trait Gradient<T> {
    type Gradient;

    fn grad(&self, args: T) -> Self::Gradient;
}

pub trait IsDifferentiable {
    /// Returns true if the function is differentiable.
    fn differentiable(&self) -> bool;
}

impl<S> IsDifferentiable for S
where
    S: Grad,
{
    fn differentiable(&self) -> bool {
        true
    }
}
