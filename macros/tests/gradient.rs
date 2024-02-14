/*
    Appellation: gradient <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(test)]
extern crate acme_macros as macros;

use macros::gradient;

#[test]
fn test_gradient() {
    let x = 1.0;
    let y = 2.0;
    assert_eq!(gradient!(x + y), [1.0; 2]);
    // assert_eq!(gradient!(x * y), [2.0, 1.0]);
    assert_eq!(gradient!(x + y + 1.0), [1.0, 1.0, 0.0]);
}
