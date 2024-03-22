/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::TensorBase;
use acme::ops::binary::BinaryOp;
use acme::ops::unary::UnaryOp;

#[derive(Clone, Debug)]
pub enum TensorOp<T> {
    Binary(Box<TensorBase<T>>, Box<TensorBase<T>>, BinaryOp),
    BinaryScalar(Box<TensorBase<T>>, T, BinaryOp),
    Unary(Box<TensorBase<T>>, UnaryOp),
    Matmul(Box<TensorBase<T>>, Box<TensorBase<T>>),
}

pub enum Inputs<T> {
    Scalar(T),
    Tensor(TensorBase<T>),
}
