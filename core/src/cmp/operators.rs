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

impl<T> Gradient<Vec<T>> for Operator<T>
where
    T: Clone,
{
    type Gradient = Vec<T>;

    fn grad(&self, args: Vec<T>) -> Self::Gradient {
        let grad = |args: Vec<T>| {
            let mut result = Vec::new();
            for i in 0..args.len() {
                let mut args = args.clone();
                args[i] = self.inputs[i].clone();
                result.push((self.operation)(args));
            }
            result
        };
        grad(args)
    }
}

impl<T> Evaluate<Vec<T>> for Operator<T> {
    type Output = T;

    fn eval(&self, args: Vec<T>) -> Self::Output {
        (self.operation)(args)
    }
}
