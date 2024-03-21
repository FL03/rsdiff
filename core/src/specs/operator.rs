/*
    Appellation: operator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use std::marker::Tuple;

pub trait Operand<Args>
where
    Args: Tuple,
{
    type Output;

    fn name(&self) -> &str;

    fn eval(&self, args: Args) -> Self::Output;

    fn grad(&self, args: Args) -> Vec<Self::Output>;
}
