/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::TensorBase;

#[derive(Clone, Debug)]
pub enum Op<T> {
    Binary(Box<TensorBase<T>>, Box<TensorBase<T>>, BinaryOp),
    Unary(Box<TensorBase<T>>, UnaryOp),
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOp {
    Add,
    Div,
    Matmul,
    Mul,
    Sub,
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOp {
    Abs,
    Cos,
    Cosh,
    Exp,
    Log,
    Ln,
    Neg,
    Reciprocal,
    Sin,
    Sinh,
    Sqrt,
    Square,
    Tan,
    Tanh,
}

pub struct BinOp<T> {
    pub lhs: TensorBase<T>,
    pub rhs: TensorBase<T>,
    pub op: BinaryOp,
}
