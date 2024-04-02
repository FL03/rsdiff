/*
    Appellation: stats <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use crate::shape::Axis;

pub trait SummaryStatistics<T> {
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
    /// Compute the standard deviation
    fn std(&self) -> T;
    /// Compute the variance
    fn variance(&self) -> T;
}

pub trait TensorStats<T>: SummaryStatistics<T> {
    
    /// Compute the mean along the specified axis.
    fn mean_axis(&self, axis: Axis) -> T;
}

#[cfg(test)]
mod tests {}
