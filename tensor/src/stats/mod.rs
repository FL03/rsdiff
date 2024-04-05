/*
    Appellation: stats <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

mod impl_stats;

use crate::prelude::{Axis, Scalar};
#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap;
#[cfg(feature = "std")]
use std::collections::BTreeMap;

// pub trait StatElem: Copy + FromPrimitive + Product + Num + NumAssign + Ord + Sum {}

// impl<S> StatElem for S where S: Copy + FromPrimitive + Product + Num + NumAssign + Ord + Sum {}

pub trait Statistics<T> {
    /// Returns the maximum value in the collection.
    fn max(&self) -> T;
    /// Returns the mean (average) value of the collection.
    fn mean(&self) -> T;
    /// Returns the median value in the collection.
    fn median(&self) -> T;
    /// Returns the minimum value in the collection.
    fn min(&self) -> T;
    /// Get the mode of the collection.
    fn mode(&self) -> T;

    fn sum(&self) -> T;
    /// Compute the standard deviation
    fn std(&self) -> T;
    /// Compute the variance
    fn variance(&self) -> T;
}

macro_rules! impl_stats {
    ($container:ty, $size:ident) => {
        impl<T> Statistics<T> for $container
        where
            Self: Clone,
            T: Ord + Scalar,
        {
            fn max(&self) -> T {
                self.iter().max().unwrap().clone()
            }

            fn mean(&self) -> T {
                self.sum() / T::from_usize(self.$size()).unwrap()
            }

            fn median(&self) -> T {
                let mut sorted = self.clone();
                sorted.sort();
                let mid = sorted.$size() / 2;
                if sorted.$size() % 2 == 0 {
                    (sorted[mid - 1] + sorted[mid]) / T::from_usize(2).unwrap()
                } else {
                    sorted[mid]
                }
            }

            fn min(&self) -> T {
                self.iter().min().unwrap().clone()
            }

            fn mode(&self) -> T {
                let mut freqs = BTreeMap::new();
                for &val in self.iter() {
                    *freqs.entry(val).or_insert(0) += 1;
                }
                let max_freq = freqs.values().max().unwrap();
                *freqs.iter().find(|(_, &freq)| freq == *max_freq).unwrap().0
            }

            fn sum(&self) -> T {
                self.iter().copied().sum()
            }

            fn std(&self) -> T {
                self.variance().sqrt()
            }

            fn variance(&self) -> T {
                let sqr = |x| x * x;
                let mean = self.mean();
                self.iter().map(|x| sqr(*x - mean)).sum::<T>()
                    / T::from_usize(self.$size()).unwrap()
            }
        }
    };
}
impl_stats!(Vec<T>, len);
impl_stats!([T], len);
pub trait StatisticsExt<T>: Statistics<T> {
    /// Compute the mean along the specified axis.
    fn mean_axis(&self, axis: Axis) -> T;
}

pub(crate) mod prelude {
    pub use super::{Statistics, StatisticsExt};
}

#[cfg(test)]
mod tests {}
