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

pub struct Node<T> {
    args: Vec<Variable<T>>,
}
