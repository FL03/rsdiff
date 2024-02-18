/*
    Appellation: gradient <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused_variables)]
#[cfg(test)]
extern crate acme_macros as macros;

use approx::assert_abs_diff_eq;
use macros::autodiff;
use num::traits::Float;
use std::ops::Add;

pub fn add<A, B, C>(x: A, y: B) -> C
where
    A: Add<B, Output = C>,
{
    x + y
}

pub fn mixed(x: f64, y: f64) -> f64 {
    y * (x + y)
}

pub fn sigmoid<T>(x: T) -> T
where
    T: Float,
{
    (T::one() + x.neg().exp()).recip()
}

pub fn sigmoid_prime<T>(x: T) -> T
where
    T: Float,
{
    x.neg().exp() / (T::one() + x.neg().exp()).powi(2)
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

#[test]
fn test_autodiff() {
    let (x, y) = (1.0, 2.0);
    // differentiating a function item w.r.t. a
    assert_eq!(
        autodiff!(a: fn addition(a: f64, b: f64) -> f64 { a + b }),
        1.0
    );
    // differentiating a closure item w.r.t. x
    assert_eq!(autodiff!(x: | x: f64, y: f64 | x * y ), 2.0);
    // differentiating a function call w.r.t. x
    assert_eq!(autodiff!(x: add(x, y)), 1.0);
    // differentiating a function call w.r.t. some variable
    assert_eq!(autodiff!(a: add(x, y)), 0.0);
    // differentiating a method call w.r.t. the reciever (x)
    assert_eq!(autodiff!(x: x.add(y)), 1.0);
    // differentiating an expression w.r.t. x
    assert_eq!(autodiff!(x: x + y), 1.0);
    assert_eq!(autodiff!(y: x += y), 1.0);
}

#[test]
fn test_array() {
    let x = [1.0, 2.0];
    let y = [2.0, 2.0];
    assert_eq!(autodiff!(x: x + y), 1.0);
}

#[test]
fn test_add() {
    let x = [1.0];
    let y = 2.0;
    assert_eq!(autodiff!(x: x + y), 1.0);
    assert_eq!(autodiff!(y: x += y), 1.0);
}

#[test]
fn test_div() {
    let x = 1.0;
    let y = 2.0;
    assert_eq!(autodiff!(x: x / y), 1.0 / 2.0);
    assert_eq!(autodiff!(y: x / y), -1.0 / 4.0);
    assert_eq!(autodiff!(x: x /= y), 1.0 / 2.0);
    assert_eq!(autodiff!(y: x /= y), -1.0 / 4.0);
}

#[test]
fn test_mul() {
    let x = 1.0;
    let y = 2.0;
    assert_eq!(autodiff!(x: x * y), 2.0);
    assert_eq!(autodiff!(y: x * y), 1.0);
    assert_eq!(autodiff!(x: x *= y), 2.0);
    assert_eq!(autodiff!(y: x *= y), 1.0);
    assert_eq!(autodiff!(y: x * y + 3.0), 1.0);
}

#[test]
fn test_sub() {
    let x = 1.0;
    let y = 2.0;
    assert_eq!(autodiff!(x: x - y), 1.0);
    assert_eq!(autodiff!(y: x - y), -1.0);
    assert_eq!(autodiff!(x: x -= y), 1.0);
    assert_eq!(autodiff!(y: x -= y), -1.0);
}

#[test]
fn test_foil() {
    let (x, y) = (1_f64, 2_f64);

    assert_eq!(autodiff!(x: (x + y) * (x + y)), 2_f64 * (x + y));
    assert_eq!(autodiff!(x: (x + y) * (x + y)), autodiff!(y: (x + y) * (x + y)));
}

#[test]
fn test_mixed_order() {
    let x = 1.0;
    let y = 2.0;
    assert_eq!(autodiff!(x: y * (x + y)), 2.0);
    assert_eq!(autodiff!(y: y * (x + y)), 5.0);
    assert_eq!(autodiff!(x: (x + y) * y), 2.0);
    assert_eq!(autodiff!(y: (x + y) * y), 5.0);
}

#[test]
fn test_trig() {
    let x: f64 = 2.0;
    assert_eq!(autodiff!(x: x.cos()), -x.sin());
    assert_eq!(autodiff!(x: x.sin()), x.cos());
    assert_eq!(autodiff!(x: x.tan()), x.cos().square().recip());
}

#[test]
fn test_log() {
    let x: f64 = 2.0;

    assert_eq!(autodiff!(x: x.ln()), 2_f64.recip());
    assert_eq!(autodiff!(x: (x + 1.0).ln()), 3_f64.recip());
}



#[test]
fn test_chained() {
    let x: f64 = 2.0;
    assert_abs_diff_eq!(autodiff!(x: x.sin() * x.cos()), 2_f64 * x.cos().square() - 1_f64, epsilon = 1e-8);
    assert_eq!(autodiff!(x: x.sin().cos()), -x.cos() * x.sin().sin());
    assert_eq!(autodiff!(x: x.ln().ln()), (x * x.ln()).recip());
}

#[test]
fn test_sigmoid() {
    let x = 2_f64;
    assert_eq!(autodiff!(x: 1.0 / (1.0 + (-x).exp())), sigmoid_prime(x));
    assert_eq!(autodiff!(x: | x: f64 | 1.0 / (1.0 + (-x).exp())), sigmoid_prime(x));
}

#[ignore = "Currently, support for function calls is not fully implemented"]
#[test]
fn test_function_call() {
    let x = 2_f64;
    assert_eq!(autodiff!(x: sigmoid(x)), sigmoid_prime(x));
}

#[ignore = "Custom trait methods are not yet supported"]
#[test]
fn test_method() {
    let x = 2_f64;
    assert_eq!(autodiff!(x: x.sigmoid()), sigmoid_prime(x));
}