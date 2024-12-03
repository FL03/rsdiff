/*
    Appellation: scg <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]
extern crate rsdiff_graphs as graphs;

use graphs::scg::Scg;

#[test]
#[ignore = "Test fails; needs to be fixed"]
fn test_scg() {
    let mut dag = Scg::new();
    let x = dag.variable(1f64);
    let y = dag.variable(2f64);
    // f(x, y) = x + y
    let c = dag.add(x, y).unwrap();
    // verify the value of c to be the sum of x and y
    assert_eq!(*dag.get_value(c).unwrap(), 3.0);
    // f(x, y) = y * (x + y)
    let d = dag.mul(c, y).unwrap();
    // verify the value of d to be the product of c and y
    assert_eq!(*dag.get_value(d).unwrap(), 6.0);

    let gc = dag.gradient_at(c).unwrap();

    assert_eq!(gc[&x], 1.0);
    assert_eq!(gc[&y], 1.0);

    let gd = dag.backward().unwrap();

    assert_eq!(gd[&x], 2.0);
    assert_eq!(gd[&y], 5.0);
}

#[test]
#[ignore = "This test is failing"]
fn test_backward() {
    let mut dag = Scg::new();
    let x = dag.variable(1f64);
    let y = dag.variable(2f64);

    let c = dag.add(x, y).unwrap();

    let d = dag.mul(c, y).unwrap();

    assert_eq!(*dag.get_value(c).unwrap(), -1.0);
    assert_eq!(*dag.get_value(d).unwrap(), -2.0);

    let gc = dag.gradient_at(c).unwrap();

    assert_eq!(gc[&x], 1.0);
    assert_eq!(gc[&y], -1.0);

    let gd = dag.backward().unwrap();

    assert_eq!(gd[&x], 2.0);
    assert_eq!(gd[&y], -3.0);
}
