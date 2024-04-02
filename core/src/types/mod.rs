/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Types
//!
//!
pub use self::{constants::*, dual::*, variables::*};

pub(crate) mod constants;
pub(crate) mod dual;
pub(crate) mod variables;

/// A boxed error type for use in the library.
#[cfg(feature = "std")]
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
/// A boxed result type for use in the library.
#[cfg(feature = "std")]
pub type BoxResult<T = ()> = core::result::Result<T, BoxError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant() {
        let a = Constant(3);
        let add = a + 3;
        assert_eq!(add, Constant(6));

        // let b = Constant(3_f64).ln();

        let a = Constant::new(3);
        let b = Constant::new(3);
        assert_eq!(a + b, Constant(6));
    }
}
