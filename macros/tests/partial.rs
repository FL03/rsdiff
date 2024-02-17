/*
    Appellation: gradient <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(test)]
extern crate acme_macros as macros;

use macros::partial;

macro_rules! partials {
    ($($x:ident),* : $f:expr) => {
        {
            let mut store = Vec::new();
            $(
                store.push(partial!($x: $f));
            )*
            store
        }
    };
}

#[test]
fn test_partial_add() {
    let x = 1.0;
    let y = 2.0;
    assert_eq!(partial!(x: x + y), 1.0);
    assert_eq!(partial!(y: x += y), 1.0);
    assert_eq!(partials!(x, y: x + y + 3.0), [1.0; 2]);
}

#[test]
fn test_partial_div() {
    let x = 1.0;
    let y = 2.0;
    assert_eq!(partial!(x: x / y), 1.0 / 2.0);
    assert_eq!(partial!(y: x / y), -1.0 / 4.0);
}

#[test]
fn test_partial_mul() {
    let x = 1.0;
    let y = 2.0;
    assert_eq!(partial!(x: x * y), 2.0);
    assert_eq!(partial!(y: x * y), 1.0);
    assert_eq!(partial!(y: x * y + 3.0), 1.0);
}

#[test]
fn test_partial_sub() {
    let x = 1.0;
    let y = 2.0;
    assert_eq!(partial!(x: x - y), 1.0);
    assert_eq!(partial!(y: x - y), -1.0);
}

#[test]
fn test_partial_mixed() {
    let x = 1.0;
    let y = 2.0;
    assert_eq!(partial!(x: y * (x + y)), 2.0);
    assert_eq!(partial!(y: y * (x + y)), 5.0);
    assert_eq!(partial!(x: (x + y) * y), 2.0);
    assert_eq!(partial!(y: (x + y) * y), 5.0);
}
