/*
    Appellation: addition <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::{Evaluate, Gradient};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Addition<T>(T, T);

impl<T> Addition<T> {
    pub fn new(a: T, b: T) -> Self {
        Self(a, b)
    }
}

impl<S, T> Evaluate for Addition<S>
where
    S: Evaluate,
    S::Output: std::ops::Add<Output = T>,
{
    type Output = T;

    fn eval(self) -> Self::Output {
        self.0.eval() + self.1.eval()
    }
}

impl<T> Gradient<T> for Addition<T>
where
    T: Clone + Gradient<T> + std::ops::Add<Output = T>,
{
    type Gradient = Addition<T::Gradient>;

    fn grad(&self, args: T) -> Self::Gradient {
        Addition(self.0.grad(args.clone()), self.1.grad(args))
    }
}
