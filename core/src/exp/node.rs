/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Node
//!
//! A computational graph relies on weighted nodes to represent constants, operations, and variables.
//! The edges connecting to any given node are considered to be inputs and help to determine the flow of information
use crate::cmp::Variable;
use crate::ops::Op;

pub enum Node<T> {
    Const(T),
    Var(Variable<T>),
    Op(Op<Variable<T>>),
}

impl<T> Node<T> {
    pub fn constant(data: T) -> Self {
        Self::Const(data)
    }

    pub fn variable(name: impl ToString, value: Option<T>) -> Self {
        Self::Var(Variable::new(name, value))
    }

    pub fn operation(op: Op<Variable<T>>) -> Self {
        Self::Op(op)
    }

    
}