/*
    Appellation: gradient <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use super::store::Store;

pub trait IsDifferentiable {
    fn differentiable(&self) -> bool;
}

pub trait Differentiable<T> {
    type Derivative;

    fn diff(&self, args: T) -> Self::Derivative;
}

pub trait Gradient<T> {
    type Gradient;

    fn grad(&self, args: T) -> Self::Gradient;
}

pub trait Grad<T> {
    type Gradient: Store<usize, T>;

    fn grad(&self) -> Self::Gradient;
}

pub trait Partial {
    type Args;
    type Output;

    fn partial(&self) -> fn(Self::Args) -> Self::Output;

    fn partial_at(&self, args: Self::Args) -> Self::Output {
        (self.partial())(args)
    }
}

pub trait Parameter {
    type Key;
    type Value;

    fn key(&self) -> Self::Key;
    fn value(&self) -> Self::Value;
}
