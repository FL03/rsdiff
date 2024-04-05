/*
    Appellation: arange <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::utils::steps;
use num::traits::{FromPrimitive, Num, ToPrimitive};

pub struct Arange<T> {
    scope: usize,
    start: T,
    stop: T,
    step: T,
}

impl<T> Arange<T> {
    pub fn new(start: T, stop: T, step: T) -> Self {
        Self {
            scope: 0,
            start,
            stop,
            step,
        }
    }

    pub fn start(&self) -> &T {
        &self.start
    }

    pub fn stop(&self) -> &T {
        &self.stop
    }

    pub fn step(&self) -> &T {
        &self.step
    }

    pub fn steps(&self) -> usize
    where
        T: Copy + Num + ToPrimitive,
    {
        steps(self.start, self.stop, self.step)
    }
}

impl<T> Iterator for Arange<T>
where
    T: Copy + FromPrimitive + Num + ToPrimitive,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.scope < self.steps() {
            let value = self.start + self.step * T::from_usize(self.scope).unwrap();
            self.scope += 1;
            Some(value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arange() {
        let mut arange = Arange::new(0, 10, 2);
        assert_eq!(arange.next(), Some(0));
        assert_eq!(arange.next(), Some(2));
        assert_eq!(arange.next(), Some(4));
        assert_eq!(arange.next(), Some(6));
        assert_eq!(arange.next(), Some(8));
        assert_eq!(arange.next(), None);
    }
}
