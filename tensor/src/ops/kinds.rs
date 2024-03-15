/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::TensorBase;

pub trait TensorOp {}

pub enum Op<T> {
    Binary(Box<TensorBase<T>>, Box<TensorBase<T>>, BinaryOp),
    Unary(Box<TensorBase<T>>, UnaryOp),
}

pub enum BinaryOp {
    Add,
    Div,
    Mul,
    Sub,
}

pub enum UnaryOp {}

pub enum Expr<T> {
    Binary(BinaryOp),
    Unary(UnaryOp),
    Scalar(T),
    Tensor(TensorBase<T>),
}
