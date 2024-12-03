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

pub trait FnArgs {
    
    type Kind;
}

pub struct Partial<T, F> {
    pub args: T,
    pub func: F
}

impl<T, F> Partial<T, F> where F: for <'a> Fn(&'a T) -> T {
    pub fn new(args: T, func: F) -> Self {
        Self { args, func }
    }
}