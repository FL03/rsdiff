/*
    Appellation: macros <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused)]
#![cfg(all(test, feature = "macros"))]

extern crate acme;

use acme::prelude::autodiff;
use num::Float;

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
    T: Copy + std::ops::Mul<Output = T>,
{
    fn square(self) -> Self {
        self * self
    }
}

#[ignore = "Currently, support for function calls is not fully implemented"]
#[test]
fn test_function_call() {
    let (x, y) = (1_f64, 2_f64);
    // differentiating a function call w.r.t. x
    assert_eq!(autodiff!(x: add(x, y)), 1.0);
    // differentiating a function call w.r.t. some variable
    assert_eq!(autodiff!(a: add(x, y)), 0.0);
    assert_eq!(autodiff!(y: sigmoid::<f64>(y)), sigmoid_prime(y));
}

#[ignore = "Custom trait methods are not yet supported"]
#[test]
fn test_method() {
    let (x, y) = (1_f64, 2_f64);
    assert_eq!(autodiff!(x: x.mul(y)), 2.0);
    assert_eq!(autodiff!(x: x.square()), 2.0);
    assert_eq!(autodiff!(x: x.sigmoid()), sigmoid_prime(x));
}
