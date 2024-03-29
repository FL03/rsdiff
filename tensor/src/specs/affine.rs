/*
    Appellation: affine <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// [Affine] describes a type of geometric transformation which preserves
/// lines and parallelisms.
///
/// ### General Formula
/// f(x) = A * x + b
pub trait Affine<A, B> {
    type Output;

    fn affine(&self, mul: A, add: B) -> Self::Output;
}

impl<S, A, B, C> Affine<A, B> for S
where
    S: Clone + std::ops::Mul<A, Output = C>,
    C: std::ops::Add<B, Output = C>,
{
    type Output = C;

    fn affine(&self, mul: A, add: B) -> Self::Output {
        self.clone() * mul + add
    }
}
