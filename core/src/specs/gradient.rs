/*
    Appellation: gradient <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use super::store::Store;

pub trait IsDifferentiable {
    /// Returns true if the function is differentiable.
    fn differentiable(&self) -> bool;
}

pub trait Gradient<T> {
    type Gradient;

    fn grad(&self, args: T) -> Self::Gradient;
}

pub trait Grad<T> {
    type Gradient: Store<usize, T>;

    fn grad(&self) -> Self::Gradient;
}

pub trait Parameter {
    type Key;
    type Value;

    fn key(&self) -> &Self::Key;

    fn value(&self) -> &Self::Value;
}
