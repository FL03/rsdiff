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

// Mathematically, the gradient of a function is a vector of partial derivatives.

pub struct Derivative<T> {
    pub wrt: T,
    pub f: Box<dyn Fn(T) -> T>,
}

impl<T> Differentiable<T> for Derivative<T> {
    type Derivative = T;

    fn diff(&self, args: T) -> Self::Derivative {
        (self.f)(args)
    }
}
