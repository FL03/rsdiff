/*
    Appellation: expr <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Tensor;
use acme::ops::{BinaryOp, UnaryOp};
use ndarray::RawData;

pub type BoxTensor<S> = Box<Tensor<S>>;

pub enum TensorExpr<S> where S: RawData {
    Binary {
        lhs: BoxTensor<S>,
        rhs: BoxTensor<S>,
        op: BinaryOp,
    },
    Unary {
        recv: BoxTensor<S>,
        op: UnaryOp,
    }
}