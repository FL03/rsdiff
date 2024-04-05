/*
    Appellation: specs <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{moves::*, ndtensor::*, scalar::*};

pub(crate) mod moves;
pub(crate) mod ndtensor;
pub(crate) mod scalar;

pub(crate) mod prelude {
    pub use super::moves::*;
    pub use super::ndtensor::*;
    pub use super::scalar::*;
}

#[cfg(test)]
mod tests {
    use super::scalar::Scalar;
    use num::Complex;

    #[test]
    fn test_scalar() {
        let a = 3f64;
        let b = Complex::new(4f64, 0f64);

        assert_eq!(Scalar::sqr(a), 9f64);
        assert_eq!(Scalar::sqrt(b), 2f64.into());
    }
}
