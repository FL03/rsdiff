/*
    Appellation: arith <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
#![allow(unused_variables)]
extern crate acme_macros as acme;

use acme::autodiff;


#[test]
fn test_add() {
    let (x, y) = (1f64, 2f64);
    assert_eq!(autodiff!(x: x + y), 1.0);
    assert_eq!(autodiff!(y: x += y), 1.0);
    assert_eq!(autodiff!(x: x.add(y)), 1.0);
}

#[test]
fn test_div() {
    let (x, y) = (1f64, 2f64);

    assert_eq!(autodiff!(x: x / y), 0.5);
    assert_eq!(autodiff!(x: x.div(y)), 0.5);
    assert_eq!(autodiff!(y: x / y), -1.0 / 4.0);
    assert_eq!(autodiff!(x: x /= y), 0.5);
    assert_eq!(autodiff!(y: x /= y), -1.0 / 4.0);
}

#[test]
fn test_mul() {
    let (x, y) = (1f64, 2f64);

    assert_eq!(autodiff!(x: x * y + 10.0), y);
    assert_eq!(autodiff!(y: x.mul(y)), x);
    assert_eq!(autodiff!(x: x *= y), 2.0);
    assert_eq!(autodiff!(y: x *= y), 1.0);
    assert_eq!(autodiff!(y: x * y + 3.0), 1.0);
}

#[test]
fn test_sub() {
    let (x, y) = (1f64, 2f64);

    assert_eq!(autodiff!(x: x - y), 1.0);
    assert_eq!(autodiff!(y: x.sub(y)), -1.0);
    assert_eq!(autodiff!(x: x -= y), 1.0);
    assert_eq!(autodiff!(y: x -= y), -1.0);
}

#[test]
fn test_foil() {
    let (x, y) = (1f64, 2f64);

    assert_eq!(autodiff!(x: (x + y) * (x + y)), 2f64 * (x + y));
    assert_eq!(
        autodiff!(x: (x + y) * (x + y)),
        autodiff!(y: (x + y) * (x + y))
    );
}

#[test]
fn test_chain_rule() {
    let (x, y) = (1f64, 2f64);

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
fn test_log() {
    let x = 2f64;

    assert_eq!(autodiff!(x: x.ln()), 2f64.recip());
    assert_eq!(autodiff!(x: (x + 1.0).ln()), 3f64.recip());
}

#[test]
fn test_pow() {
    use num::traits::Pow;

    let (x, y) = (2f64, 3f64);
    assert_eq!(autodiff!(x: x.pow(y)), y * x.pow(2));
    assert_eq!(autodiff!(y: x.pow(y)), x.pow(y) * y.ln());
    assert_eq!(autodiff!(x: 2f64.pow(y)), 0f64);
    assert_eq!(autodiff!(y: 2f64.pow(y)), 2f64.pow(y) * y.ln());
}





