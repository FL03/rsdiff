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

pub struct Sigmoid<T> {
    pub x: T,
}

impl<T> Sigmoid<T> {
    pub fn new(x: T) -> Self {
        Self { x }
    }
}
