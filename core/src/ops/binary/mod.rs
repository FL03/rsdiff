/*
   Appellation: binary <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{arithmetic::*, kinds::*, operator::*, specs::*};

pub(crate) mod arithmetic;
pub(crate) mod kinds;
pub(crate) mod operator;
pub(crate) mod specs;

pub type BoxedBinOp<A, B = A, C = A> = Box<dyn BinOp<A, B, Output = C>>;

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum Bop<Kind>
where
    Kind: BinaryOperand,
{
    Custom { name: String, op: Kind },
}

#[allow(dead_code)]
pub(crate) trait BinaryOperand {
    type Args: BinArgs;
    type Output;

    fn eval(
        &self,
        lhs: <Self::Args as BinArgs>::Lhs,
        rhs: <Self::Args as BinArgs>::Rhs,
    ) -> Self::Output;
}

pub trait BinOp<A, B = A> {
    type Output;

    fn eval(&self, lhs: A, rhs: B) -> Self::Output;
}

impl<S, A, B, C> BinOp<A, B> for S
where
    S: Fn(A, B) -> C,
{
    type Output = C;

    fn eval(&self, lhs: A, rhs: B) -> Self::Output {
        self(lhs, rhs)
    }
}

impl<A, B, C> BinOp<A, B> for Box<dyn BinOp<A, B, Output = C>> {
    type Output = C;

    fn eval(&self, lhs: A, rhs: B) -> Self::Output {
        self.as_ref().eval(lhs, rhs)
    }
}

#[cfg(test)]
mod tests {}
