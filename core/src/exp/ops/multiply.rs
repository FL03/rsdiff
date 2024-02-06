/*
    Appellation: addition <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Addition;
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
    S: Evaluate<Output = T>,
    T: std::ops::Mul<Output = T>,
{
    type Output = T;

    fn eval(self) -> Self::Output {
        self.0.eval() * self.1.eval()
    }
}

impl<T> Gradient<T> for Multiply<T>
where
    T: Clone + Gradient<T> + std::ops::Add<Output = T>,
    T::Gradient: Evaluate<Output = T>,
{
    type Gradient = Addition<Multiply<T>>;

    fn grad(&self, args: T) -> Self::Gradient {
        let a = Multiply(self.0.grad(args.clone()).eval(), self.1.clone());
        let b = Multiply(self.0.clone(), self.1.grad(args).eval());
        Addition::new(a, b)
    }
}
