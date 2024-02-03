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

pub enum Op {
    Binary(BinaryOp),
    Compare(CompareOp),
    Custom(String),
    Unary,
}
