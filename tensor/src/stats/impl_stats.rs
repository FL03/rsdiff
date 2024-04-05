use super::Statistics;
use crate::prelude::Scalar;
use crate::TensorBase;

impl<T> TensorBase<T> {
    pub fn max(&self) -> &T
    where
        T: Ord,
    {
        self.iter().max().unwrap()
    }

    pub fn mean(&self) -> T
    where
        T: Scalar,
    {
        self.sum() / T::from_usize(self.size()).unwrap()
    }

    pub fn min(&self) -> &T
    where
        T: Ord,
    {
        self.iter().min().unwrap()
    }

    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.data_mut().sort();
    }

    pub fn std(&self) -> T
    where
        T: Scalar,
    {
        self.variance().sqrt()
    }

    pub fn variance(&self) -> T
    where
        T: Scalar,
    {
        let mean = self.mean();
        self.iter().map(|x| (*x - mean).powi(2)).sum::<T>() / T::from_usize(self.size()).unwrap()
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
        self.mean()
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
        self.sum()
    }

    fn std(&self) -> T {
        self.std()
    }

    fn variance(&self) -> T {
        self.variance()
    }
}
