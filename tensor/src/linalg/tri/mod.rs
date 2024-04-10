/*
    Appellation: tri <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::utils::*;

pub(crate) mod utils {
    use crate::tensor::TensorBase;
    use num::Zero;

    /// Returns the lower triangular portion of a matrix.
    pub fn tril<T>(a: &TensorBase<T>) -> TensorBase<T>
    where
        T: Clone + Zero,
    {
        let mut out = a.clone();
        for i in 0..a.shape()[0] {
            for j in i + 1..a.shape()[1] {
                out[[i, j]] = T::zero();
            }
        }
        out
    }
    /// Returns the upper triangular portion of a matrix.
    pub fn triu<T>(a: &TensorBase<T>) -> TensorBase<T>
    where
        T: Clone + Zero,
    {
        let mut out = a.clone();
        for i in 0..a.shape()[0] {
            for j in 0..i {
                out[[i, j]] = T::zero();
            }
        }
        out
    }
}
