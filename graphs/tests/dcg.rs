/*
    Appellation: dcg <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate acme_graphs as graphs;

use graphs::dcg::Dcg;

#[test]
fn test_dcg() {
    let mut dcg = Dcg::<f64>::new();
    let a = dcg.variable(2.0);
    let b = dcg.variable(3.0);
    let c = dcg.add(a, b);

    let grad = dcg.gradient(c).unwrap();
    assert_eq!(grad[&a], 1.0);

    let mut dcg = Dcg::<f64>::new();
    let a = dcg.variable(2.0);
    let b = dcg.variable(3.0);
    let c = dcg.mul(a, b);

    let grad = dcg.gradient(c).unwrap();
    assert_eq!(grad[&a], 3.0);
    assert_eq!(grad[&b], 2.0);
}

#[test]
fn test_backward() {
    let mut dcg = Dcg::<f64>::new();
    let a = dcg.variable(2.0);
    let b = dcg.variable(3.0);
    let c = dcg.add(a, b);

    let grad = dcg.backward().unwrap();
    assert_eq!(grad, dcg.gradient(c).unwrap());
}

#[test]
#[ignore = "Not yet implemented"]
fn test_composite_expr() {
    let mut dcg = Dcg::<f64>::new();
    let a = dcg.variable(1_f64);
    let b = dcg.variable(2_f64);
    let c = dcg.add(a, b);
    let _d = dcg.mul(c, b);

    let grad = dcg.backward().unwrap();
    assert_eq!(grad[&a], 2_f64);
    assert_eq!(grad[&b], 5_f64);
}
