/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Node
//!
//! A computational graph relies on weighted nodes to represent constants, operations, and variables.
//! The edges connecting to any given node are considered to be inputs and help to determine the flow of information
use super::Id;
use crate::cmp::Variable;
use crate::ops::Evaluate;

pub struct NodeOp {
    args: Vec<Id>,
    name: String,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Node<T> {
    Const(T),
    Var(Variable<T>),
    Op { args: Vec<Id>, name: String },
}

impl<T> Node<T> {
    pub fn constant(value: T) -> Self {
        Self::Const(value)
    }

    pub fn op(name: impl ToString, args: Vec<Id>) -> Self {
        Self::Op {
            args,
            name: name.to_string(),
        }
    }

    pub fn variable(name: impl ToString) -> Self {
        Self::Var(Variable::symbolic(name))
    }
}

// impl<T> Node<T>
// where
//     T: Clone + Default,
// {
//     pub fn compute(&self) -> T {
//         match self {
//             Self::Const(value) => value.clone(),
//             Self::Var(var) => var.eval(),
//             Self::Op { args, name } => {
//                 let mut result = T::default();
//                 for arg in args {
//                     result = result + arg.compute();
//                 }
//                 result
//             }
//         }
//     }
// }

// impl<T> Evaluate for Node<T>
// where
//     T: Clone + Default,
// {
//     type Output = T;

//     fn eval(self) -> Self::Output {
//         match self {
//             Self::Const(value) => value,
//             Self::Var(var) => var.eval(),
//             Self::Op { args, name } => {
//                 let mut result = T::default();
//                 for arg in args {
//                     result = result + arg.eval();
//                 }
//                 result
//             }
//         }
//     }
// }
