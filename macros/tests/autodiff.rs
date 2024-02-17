/*
    Appellation: gradient <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(test)]
extern crate acme_macros as macros;

use macros::autodiff;
// use std::ops::Add;

trait Square {
    fn square(self) -> Self;
}

impl Square for f64 {
    fn square(self) -> Self {
        self * self
    }
}

fn add(x: f64, y: f64) -> f64 {
    x + y
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
    // differentiating a function call w.r.t. a
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
fn test_mixed_order() {
    let x = 1.0;
    let y = 2.0;
    assert_eq!(autodiff!(x: y * (x + y)), 2.0);
    assert_eq!(autodiff!(y: y * (x + y)), 5.0);
    assert_eq!(autodiff!(x: (x + y) * y), 2.0);
    assert_eq!(autodiff!(y: (x + y) * y), 5.0);
}

#[test]
fn test_complex() {
    let x: f64 = 2.0;
    assert_eq!(autodiff!(x: x.ln()), 2_f64.recip());
    assert_eq!(autodiff!(x: (x + 1.0).ln()), 3_f64.recip());
    assert_eq!(autodiff!(x: x.cos()), -x.sin());
    assert_eq!(autodiff!(x: x.sin()), x.cos());
    assert_eq!(autodiff!(x: x.tan()), x.cos().square().recip());
}
