/*
    Appellation: evaluate <traits>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::{Operator, Params};

pub trait Evaluate<Args>
where
    Self: Operator,
    Args: Params,
{
    type Output;

    fn eval(&self, args: Args) -> Self::Output;
}

pub trait Differentiable<Args>
where
    Self: Evaluate<Args>,
    Args: Params,
{
    type Grad;

    fn grad(&self, args: Args) -> Self::Grad;
}
