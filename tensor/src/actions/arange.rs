/*
    Appellation: arange <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use num::traits::real::Real;
use num::traits::{One, Zero};
use std::ops;

pub trait Ranged<T> {
    fn arange(start: T, stop: T, step: T) -> Self;

    fn arange_between(start: T, stop: T) -> Self;

    fn arange_until(stop: T) -> Self;
}

pub trait Linstep {
    type Elem;

    fn linstep(start: Self::Elem, stop: Self::Elem, steps: usize) -> Vec<Self::Elem>;
}

pub enum Ranges<T> {
    Arange { start: T, stop: T },
    Between { start: T, stop: T },
    Until { stop: T },
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Aranged<T> {
    Arange { start: T, stop: T, step: T },
    Between { start: T, stop: T },
    Until { stop: T },
}

impl<T> Aranged<T>
where
    T: Copy,
{
    /// Returns the start value of the range.
    pub fn start(&self) -> T
    where
        T: Zero,
    {
        match self {
            Aranged::Arange { start, .. } => *start,
            Aranged::Between { start, .. } => *start,
            Aranged::Until { .. } => T::zero(),
        }
    }
    /// Returns the stop value of the range.
    pub fn stop(&self) -> T {
        match self {
            Aranged::Arange { stop, .. } => *stop,
            Aranged::Between { stop, .. } => *stop,
            Aranged::Until { stop } => *stop,
        }
    }
    /// Returns the step value of the range.
    pub fn step(&self) -> T
    where
        T: One,
    {
        match self {
            Aranged::Arange { step, .. } => *step,
            Aranged::Between { .. } => T::one(),
            Aranged::Until { .. } => T::one(),
        }
    }
    /// Returns the number of steps between the given boundaries
    pub fn steps(&self) -> usize
    where
        T: Real,
    {
        match self {
            Aranged::Arange { start, stop, step } => {
                let n = ((*stop - *start) / *step).ceil().to_usize().unwrap();
                n
            }
            Aranged::Between { start, stop } => {
                let n = (*stop - *start).to_usize().unwrap();
                n
            }

            Aranged::Until { stop } => {
                let n = stop.to_usize().unwrap();
                n
            }
        }
    }
}

impl<T> From<ops::Range<T>> for Aranged<T> {
    fn from(args: ops::Range<T>) -> Self {
        Aranged::Between {
            start: args.start,
            stop: args.end,
        }
    }
}

impl<T> From<ops::RangeTo<T>> for Aranged<T> {
    fn from(args: ops::RangeTo<T>) -> Self {
        Aranged::Until { stop: args.end }
    }
}

impl<T> From<(T, T, T)> for Aranged<T> {
    fn from(args: (T, T, T)) -> Self {
        Aranged::Arange {
            start: args.0,
            stop: args.1,
            step: args.2,
        }
    }
}

impl<T> From<[T; 3]> for Aranged<T>
where
    T: Copy,
{
    fn from(args: [T; 3]) -> Self {
        Aranged::Arange {
            start: args[0],
            stop: args[1],
            step: args[2],
        }
    }
}

impl<T> From<(T, T)> for Aranged<T> {
    fn from(args: (T, T)) -> Self {
        Aranged::Between {
            start: args.0,
            stop: args.1,
        }
    }
}

impl<T> From<T> for Aranged<T> {
    fn from(stop: T) -> Self {
        Aranged::Until { stop }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arange_args() {
        let arange = Aranged::Between { start: 0, stop: 10 };
        assert_eq!(arange.start(), 0);
        assert_eq!(arange.stop(), 10);
        assert_eq!(arange.step(), 1);
        assert_eq!(arange, (0..10).into());
    }
}
