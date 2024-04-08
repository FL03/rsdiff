/*
    Appellation: external <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
#![allow(unused)]
extern crate acme_macros as macros;

use defs::*;
use macros::autodiff;


#[test]
#[ignore = "Externally defined logic is not yet supported in the macro"]
fn test_closure() {
    let (x, y) = (1f64, 2f64);
    let mul = |x: f64, y: f64| x * y;
    assert_eq!(autodiff!(x: mul(x, y)), y);
}

#[test]
#[ignore = "Externally defined logic is not yet supported in the macro"]
fn test_func() {
    let (x, y) = (1f64, 2f64);
    assert_eq!(autodiff!(x: mul(x, y)), y);
}

mod defs {
    use num::Float;

    pub fn mul<A, B, C>(x: A, y: B) -> C
    where
        A: core::ops::Mul<B, Output = C>,
    {
        x * y
    }

    pub fn sigmoid<T>(x: T) -> T
    where
        T: Float,
    {
        T::one() / (T::one() + x.neg().exp())
    }

    pub fn sigmoid_prime<T>(x: T) -> T
    where
        T: Float,
    {
        x.neg().exp() / (T::one() + x.neg().exp()).powi(2)
    }
}
