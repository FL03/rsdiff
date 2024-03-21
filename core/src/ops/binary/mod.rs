/*
   Appellation: binary <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::kinds::*;

pub(crate) mod kinds;

pub trait BinaryOperator<A, B> {
    type Output;

    fn apply(lhs: A, rhs: B) -> Self::Output;
}

pub struct BinaryO<A, B> {
    pub args: (A, B),
    pub op: BinaryOp,
}

#[cfg(test)]
mod tests {}
