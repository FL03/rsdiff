/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use syn::Expr;

pub struct Node {
    id: usize,
    expr: Box<Expr>,
}
