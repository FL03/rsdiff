/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Operations
//!
//!
pub use self::{arithmetic::*, kinds::*};

pub(crate) mod arithmetic;
pub(crate) mod kinds;

pub trait BinaryOperation<A, B = A> {
    type Output;

    fn eval(&self, lhs: A, rhs: B) -> Self::Output;
}

impl<S, A, B, C> BinaryOperation<A, B> for S
where
    S: Fn(A, B) -> C,
{
    type Output = C;

    fn eval(&self, lhs: A, rhs: B) -> Self::Output {
        self(lhs, rhs)
    }
}

impl<A, B, C> BinaryOperation<A, B> for Box<dyn BinaryOperation<A, B, Output = C>> {
    type Output = C;

    fn eval(&self, lhs: A, rhs: B) -> Self::Output {
        self.as_ref().eval(lhs, rhs)
    }
}

pub trait Operator {
    fn boxed(self) -> Box<dyn Operator>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
    fn name(&self) -> String;
}

impl Operator for Box<dyn Operator> {
    fn name(&self) -> String {
        self.as_ref().name()
    }
}
