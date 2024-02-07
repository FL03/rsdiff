/*
    Appellation: hkt <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Higher Kinded Types
//!
//!

pub mod applicative;
pub mod functor;
pub mod monad;

use std::rc::Rc;
use std::sync::Arc;

pub trait HKT<U> {
    type C; // Current Type
    type T; // Type C swapped with U
}

macro_rules! impl_hkt {
    ($t:ident) => {
        impl<T, U> HKT<U> for $t<T> {
            type C = T;
            type T = $t<U>;
        }
    };
}

impl_hkt!(Arc);
impl_hkt!(Box);
impl_hkt!(Option);
impl_hkt!(Rc);
impl_hkt!(Vec);

#[cfg(test)]
mod tests {

    use super::*;
    use super::functor::Functor;

    #[test]
    fn test_hkt() {
        let v = Vec::from_iter(0..9);
        let v2 = v.map(|x| (x + 1).to_string());
        assert_eq!(v2, vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"]);
    }

}
