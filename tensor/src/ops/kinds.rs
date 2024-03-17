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
    Neg,
    Reciprocal,
    Sin,
    Sinh,
    Tan,
    Tanh,
}

pub enum Expr<T> {
    Binary(BinaryOp),
    Unary(UnaryOp),
    Scalar(T),
    Tensor(TensorBase<T>),
}

pub struct BinOp<T> {
    pub lhs: TensorBase<T>,
    pub rhs: TensorBase<T>,
    pub op: BinaryOp,
}
