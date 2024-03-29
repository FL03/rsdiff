/*
    Appellation: eval <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::evaluator::*;

pub(crate) mod evaluator;

pub trait EvaluateOnce {
    type Output;

    fn eval_once(self) -> Self::Output;
}

pub trait EvaluateMut: EvaluateOnce {
    fn eval_mut(&mut self) -> Self::Output;
}

pub trait Evaluate: EvaluateMut {
    fn eval(&self) -> Self::Output;
}

impl EvaluateOnce for f64 {
    type Output = f64;

    fn eval_once(self) -> Self::Output {
        self
    }
}
