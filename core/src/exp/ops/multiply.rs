/*
    Appellation: addition <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::{Evaluate, Gradient};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Multiply<T>(T, T);

impl<T> Multiply<T> {
    pub fn new(a: T, b: T) -> Self {
        Self(a, b)
    }
}

impl<S, T> Evaluate for Multiply<S>
where
    S: Evaluate,

    S::Output: std::ops::Mul<Output = T>,
{
    type Output = T;

    fn eval(self) -> Self::Output {
        self.0.eval() * self.1.eval()
    }
}

impl<T> Gradient<T> for Multiply<T>
where
    T: Clone + Evaluate + Gradient<T> + std::ops::Mul<Output = T>,
    <T as Evaluate>::Output: std::ops::Mul<T::Gradient, Output = T::Gradient>,
    <T as Gradient<T>>::Gradient:
        std::ops::Add<Output = T::Gradient> + std::ops::Mul<Output = T::Gradient>,
{
    type Gradient = T::Gradient;

    fn grad(&self, args: T) -> Self::Gradient {
        let a = self.1.clone().eval() * self.0.grad(args.clone());
        let b = self.0.clone().eval() * self.1.grad(args);
        a + b
    }
}
