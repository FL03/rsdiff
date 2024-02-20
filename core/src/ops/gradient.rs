/*
    Appellation: gradient <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Differentiable<T> {
    type Derivative;

    fn diff(&self, args: T) -> Self::Derivative;
}

pub trait Gradient<T> {
    type Gradient;

    fn grad(&self, args: T) -> Self::Gradient;
}

