/*
    Appellation: operator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub enum OperatorKind {
    Binary,
    Unary,
    Ternary,
    Nary,
}

pub trait Operator {
    fn kind(&self) -> OperatorKind;

    fn name(&self) -> &str;
}
