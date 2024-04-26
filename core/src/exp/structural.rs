/*
   Appellation: structural <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::{Op, Params};

pub struct Adder<A, B = A> {
    args: BinaryArgs<A, B>
}

impl<A, B> Adder<A, B> {
    pub fn new(lhs: A, rhs: B) -> Self {
        Self {
            args: BinaryArgs::new(lhs, rhs)
        }
    }
}

pub trait StructuralFn {
    type Output;
}