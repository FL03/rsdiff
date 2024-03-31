/*
    Appellation: stack <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::shape::Axis;

pub trait Stack<T> {
    type Output;

    fn stack(&self, other: &T, along: Axis) -> Self::Output;
}

pub trait Hstack<T = Self> {
    type Output;

    fn hstack(&self, other: &T) -> Self::Output;
}

pub trait Vstack<T> {
    type Output;

    fn vstack(&self, other: &T) -> Self::Output;
}
