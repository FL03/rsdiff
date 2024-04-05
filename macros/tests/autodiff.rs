/*
    Appellation: autodiff <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
#![allow(unused_variables)]
extern crate acme_macros as macros;

use approx::assert_abs_diff_eq;
use macros::autodiff;

pub fn sigmoid_prime<T>(x: T) -> T
where
    T: num::Float,
{
    x.neg().exp() / (T::one() + x.neg().exp()).powi(2)
}

#[test]
fn test_autodiff() {
    let (x, y) = (1_f64, 2_f64);
    // differentiating a closure item w.r.t. x
    assert_eq!(autodiff!(x: | x: f64, y: f64 | x * y ), y);
    assert_eq!(autodiff!(y: | x: f64, y: f64 | x * y ), x);
    // differentiating a known method call w.r.t. the reciever (x)
    assert_eq!(autodiff!(x: x.add(y)), 1.0);
    // differentiating an expression w.r.t. x
    assert_eq!(autodiff!(x: x + y), 1.0);
    assert_eq!(autodiff!(x: x + x), 2.0);
    assert_eq!(autodiff!(y: x += y), 1.0);
}

#[test]
fn test_item_function() {
    let (x, y) = (1_f64, 2_f64);
    assert_eq!(
        autodiff!(x: fn mul<A, B, C>(x: A, y: B) -> C where A: std::ops::Mul<B, Output = C> { x * y }),
        y
    );
    assert_eq!(autodiff!(y: fn mul(x: f64, y: f64) -> f64 { x * y }), x);
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
    assert_eq!(autodiff!(x: x * y + 10.0), 2.0);
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
    assert_eq!(
        autodiff!(x: (x + y) * (x + y)),
        autodiff!(y: (x + y) * (x + y))
    );
}

#[test]
fn test_chain_rule() {
    let (x, y) = (1_f64, 2_f64);

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
    assert_eq!(autodiff!(x: x.tan()), x.cos().powi(2).recip());
}

#[test]
fn test_pow() {
    use num::traits::Pow;

    let (x, y) = (2f64, 3f64);
    let y_i: i32 = 3;
    assert_eq!(autodiff!(x: x.pow(y)), y * x.pow(2));
    // assert_eq!(autodiff!(y: x.pow(y)), x.pow(y) * y.ln());
    assert_eq!(autodiff!(x: 2f64.powf(y)), 0f64);
    assert_eq!(autodiff!(x: x.powi(y)), y * x.pow(2));
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
    assert_abs_diff_eq!(
        autodiff!(x: x.sin() * x.cos()),
        2_f64 * x.cos().powi(2) - 1_f64,
        epsilon = 1e-8
    );
    assert_eq!(autodiff!(x: x.sin().cos()), -x.cos() * x.sin().sin());
    assert_eq!(autodiff!(x: x.ln().ln()), (x * x.ln()).recip());
}

#[test]
fn test_sigmoid() {
    let x = 2_f64;
    assert_eq!(autodiff!(x: 1.0 / (1.0 + (-x).exp())), sigmoid_prime(x));
    assert_eq!(
        autodiff!(x: | x: f64 | 1.0 / (1.0 + (-x).exp())),
        sigmoid_prime(x)
    );
    assert_eq!(
        autodiff!(x: fn sigmoid(x: f64) -> f64 { 1_f64 / (1_f64 + (-x).exp()) }),
        sigmoid_prime(x)
    );
}
