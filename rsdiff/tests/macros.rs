/*
    Appellation: macros <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

extern crate rsdiff;

use rsdiff::prelude::autodiff;
use utils::*;

#[test]
#[ignore = "Custom function calls are not yet supported"]
fn test_function_call() {
    let (_x, y) = (1_f64, 2_f64);
    // differentiating a function call w.r.t. x
    assert_eq!(autodiff!(x: add(x, y)), 1.0);
    // differentiating a function call w.r.t. some variable
    assert_eq!(autodiff!(a: add(x, y)), 0.0);
    assert_eq!(autodiff!(y: sigmoid::<f64>(y)), sigmoid_prime(y));
}

#[test]
#[ignore = "Custom trait methods are not yet supported"]
fn test_method() {
    let (_x, _y) = (1_f64, 2_f64);
    // assert_eq!(autodiff!(x: x.sigmoid()), sigmoid_prime(x));
}

#[allow(unused)]
mod utils {
    use core::ops::Mul;
    use num::traits::{Float, Pow};
    use std::ops::FnOnce;

    pub fn sigmoid<T>(x: T) -> T
    where
        T: Float,
    {
        (T::one() + x.neg().exp()).recip()
    }

    pub trait Sigmoid {
        fn sigmoid(self) -> Self;
    }

    impl<T> Sigmoid for T
    where
        T: Float,
    {
        fn sigmoid(self) -> Self {
            (T::one() + self.neg().exp()).recip()
        }
    }

    pub fn add<A, B, C>(a: A, b: B) -> C
    where
        A: std::ops::Add<B, Output = C>,
    {
        a + b
    }

    pub fn sigmoid_prime<T>(x: T) -> T
    where
        T: Float,
    {
        x.neg().exp() / (T::one() + x.neg().exp()).powi(2)
    }

    trait Square {
        fn square(self) -> Self;
    }

    impl<T> Square for T
    where
        T: Pow<i32, Output = T>,
    {
        fn square(self) -> Self {
            self.pow(2)
        }
    }

    pub struct Sample;

    impl Sample {
        pub fn sqr<T>(x: T) -> T
        where
            T: Copy + Mul<T, Output = T>,
        {
            x * x
        }

        pub fn item_fn<T>(&self) -> fn(T) -> T
        where
            T: Copy + Mul<T, Output = T>,
        {
            Self::sqr
        }
    }
}
