/*
    Appellation: eval <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::evaluator::*;

pub(crate) mod evaluator;

pub trait Evaluate {
    type Output;

    fn eval(self) -> Self::Output;
}

impl Evaluate for f64 {
    type Output = f64;

    fn eval(self) -> Self::Output {
        self
    }
}
