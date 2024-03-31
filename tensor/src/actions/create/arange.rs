/*
    Appellation: arange <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::utils::steps;
use core::ops::{self, Range};
use num::traits::{Bounded, FromPrimitive, Num, ToPrimitive};

pub struct Arange<T> {
    range: Boundary<T>,
    step: T,
}

impl<T> Arange<T> {
    pub fn new(range: Boundary<T>, step: T) -> Self {
        Self { range, step }
    }

    pub fn range(start: T, stop: T, step: T) -> Self {
        Self::new(Boundary::Range { start, stop }, step)
    }
}
impl<T> Arange<T>
where
    T: Copy + Default + Num + PartialOrd,
{
    pub fn start(&self) -> T {
        self.range.start()
    }

    pub fn steps(&self) -> usize
    where
        T: FromPrimitive + ToPrimitive,
    {
        steps(self.start(), self.stop(), self.step)
    }

    pub fn step(&self) -> T {
        self.step
    }

    pub fn stop(&self) -> T
    where
        T: FromPrimitive + PartialOrd,
    {
        self.range.stop_or_linear()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Boundary<T = isize> {
    Range { start: T, stop: T },
    From { start: T },
    Inclusive { start: T, stop: T },
    Until { stop: T },
}

impl<T> Boundary<T>
where
    T: Copy + Default,
{
    /// Returns the start value of the range.
    pub fn start(&self) -> T {
        match self {
            Boundary::Range { start, .. } => *start,
            Boundary::From { start } => *start,
            Boundary::Inclusive { start, .. } => *start,
            Boundary::Until { .. } => T::default(),
        }
    }
    /// Returns the stop value of the range.
    pub fn stop(&self) -> Option<T> {
        match self {
            Boundary::Range { stop, .. }
            | Boundary::Inclusive { stop, .. }
            | Boundary::Until { stop } => Some(*stop),
            _ => None,
        }
    }

    pub fn step_size(&self, steps: usize) -> T
    where
        T: FromPrimitive + Num + PartialOrd,
    {
        let steps = T::from_usize(steps).unwrap();
        let start = self.start();
        let stop = self.stop_or_default();
        let step = (stop - start) / steps;
        step
    }
}

impl<T> Boundary<T>
where
    T: Copy + Default + PartialOrd,
{
    pub fn stop_or(&self, default: T) -> T {
        debug_assert!(default >= self.start());

        self.stop().unwrap_or(default)
    }

    pub fn stop_or_linear(&self) -> T
    where
        T: FromPrimitive + Num,
    {
        self.stop_or(self.start() * T::from_usize(2).unwrap())
    }

    pub fn stop_or_default(&self) -> T {
        self.stop_or(T::default())
    }

    pub fn stop_or_max(&self) -> T
    where
        T: Bounded,
    {
        self.stop_or(T::max_value())
    }
}
impl<T> From<Range<T>> for Boundary<T> {
    fn from(args: Range<T>) -> Self {
        Boundary::Range {
            start: args.start,
            stop: args.end,
        }
    }
}

impl<T> From<ops::RangeFrom<T>> for Boundary<T> {
    fn from(args: ops::RangeFrom<T>) -> Self {
        Boundary::From { start: args.start }
    }
}

impl<T> From<ops::RangeTo<T>> for Boundary<T> {
    fn from(args: ops::RangeTo<T>) -> Self {
        Boundary::Until { stop: args.end }
    }
}

impl<T> From<[T; 2]> for Boundary<T>
where
    T: Copy,
{
    fn from(args: [T; 2]) -> Self {
        Boundary::Range {
            start: args[0],
            stop: args[1],
        }
    }
}

impl<T> From<(T, T)> for Boundary<T> {
    fn from(args: (T, T)) -> Self {
        Boundary::Inclusive {
            start: args.0,
            stop: args.1,
        }
    }
}

impl<T> From<T> for Boundary<T> {
    fn from(stop: T) -> Self {
        Boundary::Until { stop }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arange() {
        let setup = Boundary::Range { start: 0, stop: 10 };
        let arange = Arange::new(setup, 1);
        assert_eq!(arange.start(), 0);
        assert_eq!(arange.stop(), 10);
        assert_eq!(arange.step(), 1);
        assert_eq!(setup, (0..10).into());
    }
}
