/*
    Appellation: scalar <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::scalar::*;

pub(crate) mod scalar;

macro_rules! Scalar {
    (complex) => {
        Scalar!(cf64)
    };
    (float) => {
        Scalar!(f64)
    };
    (cf64) => {
        Complex<f64>
    };
    (cf32) => {
        Complex<f32>
    };
    (f64) => {
        f64
    };
    (f32) => {
        f32
    };

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar() {
        let a: Scalar!(f64);
        a = 3.0;
        assert_eq!(a, 3_f64);
    }
}
