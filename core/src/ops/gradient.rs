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

pub trait Grad<T> {
    type Output;

    /// Compute the gradient of a function at a given point, with respect to a given variable.
    // TODO: Create a macro for generating parameter keys
    fn grad(&self, at: T, wrt: &str) -> Self::Output;
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
