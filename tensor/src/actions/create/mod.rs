/*
    Appellation: create <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{arange::*, linspace::*, stack::*, utils::*};

pub(crate) mod arange;
pub(crate) mod linspace;
pub(crate) mod stack;

pub(crate) mod utils {
    use core::ops::{Div, Sub};
    use num::traits::{FromPrimitive, ToPrimitive};

    pub fn step_size<T>(start: T, stop: T, steps: usize) -> T
    where
        T: FromPrimitive + Div<Output = T> + Sub<Output = T>,
    {
        (stop - start) / T::from_usize(steps).unwrap()
    }

    pub fn steps<T>(start: T, stop: T, step: T) -> usize
    where
        T: ToPrimitive + Div<Output = T> + Sub<Output = T>,
    {
        let steps = (stop - start) / step;
        steps.to_usize().unwrap()
    }
}

#[cfg(test)]
mod tests {}
