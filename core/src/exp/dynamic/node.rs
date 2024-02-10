/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Node
//!
//!
//! The edges connecting to any given node are considered to be inputs and help to determine the flow of information

pub struct Node<T> {
    data: Vec<T>,
    operation: String,
}
