/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub enum Args<T> {
    Binary(T, T),
    Custom(Vec<T>),
    Unary(T),
}

pub enum CompareOp {
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
    Ne,
}

pub enum BinaryOp {
    Add,
    Div,
    Maximum,
    Minimum,
    Mul,
    Sub,
}

pub enum UnaryOp {
    Abs,
    Ceil,
    Cos,
    Exp,
    Floor,
    Log,
    Neg,
    Reciprocal,
    Round,
    Rsqrt,
    Sin,
    Sqrt,
    Tan,
}

pub enum Op<T> {
    Binary(T, T, BinaryOp),
    Compare(T, T, CompareOp),
    Custom(String),
    Unary(T, UnaryOp),
}
