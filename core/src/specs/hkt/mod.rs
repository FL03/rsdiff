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

macro_rules! hkt {
    ($t:ident) => {
        impl<T, U> HKT<U> for $t<T> {
            type C = T;
            type T = $t<U>;
        }
    };
}

hkt!(Arc);
hkt!(Box);
hkt!(Option);
hkt!(Rc);
hkt!(Vec);

#[cfg(test)]
mod tests {

    use super::functor::Functor;
    use super::monad::Monad;

    #[test]
    fn test_hkt_vec() {
        let v = Vec::from_iter(0..9);
        let v2 = v.fmap(|x| (x + 1).to_string());
        assert_eq!(v2, vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"]);

        let v = Vec::return_(0);
        let v2 = v.bind(|x| vec![x + 1]);
        assert_eq!(v2, vec![1]);
    }
}
