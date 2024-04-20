/*
    Appellation: gradient <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::id::Identifiable;
use crate::specs::StoreExt;
pub trait Jacobian {
    type Item;

    fn jacobian(&self) -> Self::Item;
}

pub trait Partial<T> {
    type Output;

    fn partial(&self, args: T) -> Self::Output;
}

pub trait Grad {
    type Args: Identifiable;
    type Gradient: StoreExt<Self::Args>;

    fn grad(&self) -> Self::Gradient;
    fn grad_at(&self, wrt: <Self::Args as Identifiable>::Id) -> Self::Args;
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
