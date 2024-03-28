/*
    Appellation: structural <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait StructuralFn {
    type Args: StructuredArgs;
    type Output;

    fn eval(&self) -> Self::Output;
}

pub trait StructuredArgs {}

pub struct StructFunc<F, A>
where
    F: StructuralFn,
    A: StructuredArgs,
{
    args: A,
    func: F,
}
