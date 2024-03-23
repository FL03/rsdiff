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
    let a = dcg.input(true, 2.0);
    let b = dcg.input(true, 3.0);
    let c = dcg.add(a, b);

    let grad = dcg.gradient(c).unwrap();
    assert_eq!(grad[&a], 1.0);

    let mut dcg = Dcg::<f64>::new();
    let a = dcg.input(true, 2.0);
    let b = dcg.input(true, 3.0);
    let c = dcg.mul(a, b);

    let grad = dcg.gradient(c).unwrap();
    assert_eq!(grad[&a], 3.0);
    assert_eq!(grad[&b], 2.0);
}

#[test]
fn test_simple_chained() {
    let mut dcg = Dcg::<f64>::new();
    let a = dcg.input(true, 2.0);
    let b = dcg.input(true, 1.0);
    let c = dcg.add(a, b);
    let d = dcg.mul(a, c);

    let grad = dcg.gradient(d).unwrap();
    assert_eq!(grad[&a], 2.0);
    assert_eq!(grad[&b], 2.0);
}
