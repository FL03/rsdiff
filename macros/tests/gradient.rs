/*
    Appellation: gradient <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(test)]
extern crate acme_macros as macros;

use macros::grad;

#[test]
fn test_grad_addition() {
    let x = 1.0;
    let y = 2.0;
    assert_eq!(grad!(x + y), [1.0; 2]);
    let z = 3.0;
    assert_eq!(grad!(x + y + z), [1.0; 3]);
}

#[test]
fn test_grad_multiply() {
    let x = 1.0;
    let y = 2.0;
    assert_eq!(grad!(x * y), [2.0, 1.0]);
    // assert_eq!(grad!(x * y + 3.0), [2.0, 1.0]);
}
