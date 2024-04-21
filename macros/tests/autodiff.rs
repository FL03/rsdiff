/*
    Appellation: autodiff <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
#![allow(unused)]
extern crate acme_macros as macros;

use approx::assert_abs_diff_eq;
use macros::autodiff;

fn sigmoid_prime<T>(x: T) -> T
where
    T: num::Float,
{
    x.neg().exp() / (T::one() + x.neg().exp()).powi(2)
}

#[test]
fn test_autodiff() {
    let (x, y) = (1f64, 2f64);
    // differentiating a closure item w.r.t. x
    assert_eq!(autodiff!(x: | x: f64, y: f64 | x * y ), y);
    assert_eq!(autodiff!(y: | x: f64, y: f64 | x * y ), x);
    // differentiating a known method call w.r.t. the reciever (x)
    assert_eq!(autodiff!(x: x.add(y)), 1f64);
    // differentiating an expression w.r.t. x
    assert_eq!(autodiff!(x: x + y), 1f64);
    assert_eq!(autodiff!(x: x + x), 2f64);
    assert_eq!(autodiff!(y: x += y), 1f64);
    // differentiating an item fn w.r.t. y
    assert_eq!(
        autodiff!(x: fn mul<A, B, C>(x: A, y: B) -> C where A: std::ops::Mul<B, Output = C> { x * y }),
        y
    );
}

#[test]
fn test_item_function() {
    let (x, y) = (1f64, 2f64);
    assert_eq!(
        autodiff!(x: fn mul<A, B, C>(x: A, y: B) -> C where A: std::ops::Mul<B, Output = C> { x * y }),
        y
    );
}

#[test]
#[ignore = "not yet implemented"]
fn test_array() {
    let x = [1.0, 2.0];
    let y = [2.0, 2.0];
    assert_eq!(autodiff!(x: x + y), 1f64);
    // assert_eq!(autodiff!(x: x + y), [1.0, 0.0]);
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

#[test]
#[ignore = "not yet implemented"]
fn test_typing() {
    use num::traits::Pow;

    let (x, y) = (2f64, 3f64);
    let yi: i32 = y as i32;
    // assert_eq!(autodiff!(x: x.powi(yi)), y * x.pow(2));
}
