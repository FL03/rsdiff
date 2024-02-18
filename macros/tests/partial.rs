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
fn test_add() {
    let (x, y) = (1_f64, 2_f64);
    assert_eq!(partial!(x: x + y), 1.0);
    assert_eq!(partial!(y: x += y), 1.0);
    assert_eq!(partials!(x, y: x + y + 3.0), [1.0; 2]);
}

#[test]
fn test_div() {
    let (x, y) = (1_f64, 2_f64);

    assert_eq!(partial!(x: x / y), 1.0 / 2.0);
    assert_eq!(partial!(y: x / y), -1.0 / 4.0);
}

#[test]
fn test_mul() {
    let (x, y) = (1_f64, 2_f64);

    assert_eq!(partial!(x: x * y), 2.0);
    assert_eq!(partial!(y: x * y), 1.0);
    assert_eq!(partial!(y: x * y + 3.0), 1.0);
}

#[test]
fn test_sub() {
    let (x, y) = (1_f64, 2_f64);

    assert_eq!(partial!(x: x - y), 1.0);
    assert_eq!(partial!(y: x - y), -1.0);
}

#[test]
fn test_chain_rule() {
    let (x, y) = (1_f64, 2_f64);

    assert_eq!(partial!(x: y * (x + y)), y);
    assert_eq!(partial!(y: y * (x + y)), 2_f64 * y + x);
    assert_eq!(partial!(x: (x + y) * y), y);
    assert_eq!(partial!(y: (x + y) * y), 2_f64 * y + x);
}
