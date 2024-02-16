/*
    Appellation: gradient <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(test)]
extern crate acme_macros as macros;

use macros::partial;

#[test]
fn test_partial_add() {
    let x = 1.0;
    let y = 2.0;
    assert_eq!(partial!(x: x + y), 1.0);
    assert_eq!(partial!(y: x + y), 1.0);
}

#[test]
fn test_partial_mul() {
    let x = 1.0;
    let y = 2.0;
    assert_eq!(partial!(x: x * y), 2.0);
    assert_eq!(partial!(y: x * y), 1.0);
    assert_eq!(partial!(y: x * y + 3.0), 1.0);
}
