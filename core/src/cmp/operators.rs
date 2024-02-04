/*
    Appellation: operators <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::{Evaluate, Gradient};

pub struct Operator<T> {
    pub inputs: Vec<T>,
    pub operation: Box<dyn Fn(Vec<T>) -> T>,
}

impl<T> Operator<T> {
    pub fn new(inputs: Vec<T>, operation: Box<dyn Fn(Vec<T>) -> T>) -> Self {
        Self { inputs, operation }
    }
}
