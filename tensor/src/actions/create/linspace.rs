/*
    Appellation: linspace <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::step_size;
use num::traits::{FromPrimitive, Num};

pub trait Linspace<T> {
    fn linspace(start: T, stop: T, steps: usize) -> Self;
}

pub trait LinspaceExt<T>: Linspace<T> {
    fn linspace_until(stop: T, steps: usize) -> Self;
}

impl<S, T> LinspaceExt<T> for S
where
    S: Linspace<T>,
    T: Default,
{
    fn linspace_until(stop: T, steps: usize) -> Self {
        S::linspace(T::default(), stop, steps)
    }
}

impl<T> Linspace<T> for Vec<T>
where
    T: Copy + Default + FromPrimitive + Num + PartialOrd,
{
    fn linspace(start: T, stop: T, steps: usize) -> Self {
        let step = step_size(start, stop, steps);
        let mut vec = Vec::with_capacity(steps);
        let mut value = start;
        for _ in 0..steps {
            vec.push(value);
            value = value + step;
        }
        vec
    }
}
