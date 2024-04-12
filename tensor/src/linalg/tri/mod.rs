/*
    Appellation: tri <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::utils::*;

pub(crate) mod utils {
    use crate::linalg::UPLO;
    use crate::tensor::TensorBase;
    use num::Zero;

    pub fn triangular<T>(a: &TensorBase<T>, uplo: UPLO) -> TensorBase<T>
    where
        T: Clone + Zero,
    {
        debug_assert!(a.is_square(), "Tensor must be square.");
        match uplo {
            UPLO::Upper => triu(a),
            UPLO::Lower => tril(a),
        }
    }

    /// Returns the lower triangular portion of a matrix.
    pub fn tril<T>(a: &TensorBase<T>) -> TensorBase<T>
    where
        T: Clone + Zero,
    {
        debug_assert!(a.is_square(), "Matrix must be square.");
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
