/*
    Appellation: gradient <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait IsDifferentiable {
    /// Returns true if the function is differentiable.
    fn differentiable(&self) -> bool;
}

pub trait Grad {
    type Output;

    fn grad(&self) -> Self::Output;
}

pub trait Gradient<T> {
    type Gradient;

    fn grad(&self, args: T) -> Self::Gradient;
}

impl<S> IsDifferentiable for S
where
    S: Grad,
{
    fn differentiable(&self) -> bool {
        true
    }
}
