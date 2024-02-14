/*
    Appellation: gradient <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(test)]
extern crate acme_macros as macros;

use macros::partial;

#[test]
fn test_partial() {
    let x = 1.0;
    let y = 2.0;
    assert_eq!(partial!(x: x + y;), 1.0);
    assert_eq!(partial!(y: x + y;), 1.0);
    assert_eq!(partial!(x: x * y;), 2.0);
    assert_eq!(partial!(y: x * y;), 1.0);
}
