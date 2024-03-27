/*
    Appellation: arange <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use num::traits::{FromPrimitive, Num, ToPrimitive};
use std::ops;

pub struct Arange<T> {
    range: Aranged<T>,
    step: T,
}

impl<T> Arange<T> {
    pub fn new(range: Aranged<T>, step: T) -> Self {
        Self { range, step }
    }

    pub fn range(start: T, stop: T, step: T) -> Self {
        Self::new(Aranged::Range { start, stop }, step)
    }
}
impl<T> Arange<T>
where
    T: Copy + Default + Num,
{
    pub fn start(&self) -> T {
        self.range.start()
    }

    pub fn steps(&self) -> usize
    where
        T: FromPrimitive + ToPrimitive,
    {
        let start = self.range.start();
        let stop = self.range.stop();
        let step = self.step;
        let steps = (stop - start) / step;
        steps.to_usize().unwrap()
    }

    pub fn step(&self) -> T {
        self.step
    }

    pub fn stop(&self) -> T {
        self.range.stop()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Aranged<T> {
    Range { start: T, stop: T },
    Inclusive { start: T, stop: T },
    Until { stop: T },
}

impl<T> Aranged<T>
where
    T: Copy + Default,
{
    /// Returns the start value of the range.
    pub fn start(&self) -> T {
        match self {
            Aranged::Range { start, .. } => *start,
            Aranged::Inclusive { start, .. } => *start,
            Aranged::Until { .. } => T::default(),
        }
    }
    /// Returns the stop value of the range.
    pub fn stop(&self) -> T {
        match self {
            Aranged::Range { stop, .. } => *stop,
            Aranged::Inclusive { stop, .. } => *stop,
            Aranged::Until { stop } => *stop,
        }
    }

    pub fn step_size(&self, steps: usize) -> T
    where
        T: FromPrimitive + Num,
    {
        let steps = T::from_usize(steps).unwrap();
        let start = self.start();
        let stop = self.stop();
        let step = (stop - start) / steps;
        step
    }
}

impl<T> From<ops::Range<T>> for Aranged<T> {
    fn from(args: ops::Range<T>) -> Self {
        Aranged::Range {
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

impl<T> From<[T; 2]> for Aranged<T>
where
    T: Copy,
{
    fn from(args: [T; 2]) -> Self {
        Aranged::Range {
            start: args[0],
            stop: args[1],
        }
    }
}

impl<T> From<(T, T)> for Aranged<T> {
    fn from(args: (T, T)) -> Self {
        Aranged::Inclusive {
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
    fn test_arange() {
        let setup = Aranged::Range { start: 0, stop: 10 };
        let arange = Arange::new(setup, 1);
        assert_eq!(arange.start(), 0);
        assert_eq!(arange.stop(), 10);
        assert_eq!(arange.step(), 1);
        assert_eq!(setup, (0..10).into());
    }
}
