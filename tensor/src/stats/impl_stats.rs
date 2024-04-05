use super::Statistics;
use crate::prelude::Scalar;
use crate::TensorBase;

impl<T> TensorBase<T>
where
    T: Ord,
{
    pub fn max(&self) -> &T {
        self.iter().max().unwrap()
    }

    pub fn min(&self) -> &T {
        self.iter().min().unwrap()
    }

    pub fn sort(&mut self) {
        self.data_mut().sort();
    }
}

impl<T> Statistics<T> for TensorBase<T>
where
    T: Ord + Scalar,
{
    fn max(&self) -> T {
        *self.max()
    }

    fn mean(&self) -> T {
        self.sum() / T::from_usize(self.size()).unwrap()
    }

    fn median(&self) -> T {
        self.data().median()
    }

    fn min(&self) -> T {
        *self.min()
    }

    fn mode(&self) -> T {
        self.data().mode()
    }

    fn sum(&self) -> T {
        self.iter().copied().sum()
    }

    fn std(&self) -> T {
        self.variance().sqrt()
    }

    fn variance(&self) -> T {
        let mean = self.mean();
        self.iter().map(|x| (*x - mean).powi(2)).sum::<T>() / T::from_usize(self.size()).unwrap()
    }
}
