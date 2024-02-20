/*
    Appellation: structural <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait StructuralFn {
    type Output;

    fn eval(&self) -> Self::Output;
}
