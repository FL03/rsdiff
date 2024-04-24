/*
   Appellation: ternary <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::kinds::*;

pub(crate) mod kinds;

// use super::{Evaluator, Operator, Params};

pub trait ApplyTernary<A, B = A, C = A> {
    type Output;

    fn apply(&self, a: A, b: B, c: C) -> Self::Output;
}

pub struct TernaryArgs<A, B, C>(pub A, pub B, pub C);

// impl<S, P, A, B, C, D> Evaluator<P> for S
// where
//     P: Params<Pattern = (A, B, C)>,
//     S: Operator + Ternary<A, B, C, Output = D>,
// {
//     type Output = <Self as Ternary<A, B, C>>::Output;

//     fn eval(&self, args: P) -> Self::Output {
//         let (a, b, c) = args.into_pattern();
//         self.apply(a, b, c)
//     }
// }
