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
    let df = grad!(x + y);
    // let df = BTreeMap::from_iter(df);
    assert_eq!(df.into_iter().filter(|(k, _v)| k == &x).collect::<Vec<_>>(), [(x, 1.0)]);
    assert_eq!(df.into_iter().filter(|(k, _v)| k == &y).collect::<Vec<_>>(), [(y, 1.0)]);
    let z = 3.0;
    let df = grad!(x + y + z);
    assert_eq!(df.into_iter().filter(|(k, _v)| k == &x).collect::<Vec<_>>(), [(x, 1.0)]);
    assert_eq!(df.into_iter().filter(|(k, _v)| k == &y).collect::<Vec<_>>(), [(y, 1.0)]);
    assert_eq!(df.into_iter().filter(|(k, _v)| k == &z).collect::<Vec<_>>(), [(z, 1.0)]);
}

#[test]
fn test_grad_multiply() {
    let x = 1.0;
    let y = 2.0;
    let df = grad!(x * y);
    assert_eq!(df.into_iter().filter(|(k, _v)| k == &x).collect::<Vec<_>>(), [(x, 2.0)]);
    assert_eq!(df.into_iter().filter(|(k, _v)| k == &y).collect::<Vec<_>>(), [(y, 1.0)]);
    let df = grad!(x * y + 3.0);
    assert_eq!(df.into_iter().filter(|(k, _v)| k == &x).collect::<Vec<_>>(), [(x, 2.0)]);
    assert_eq!(df.into_iter().filter(|(k, _v)| k == &y).collect::<Vec<_>>(), [(y, 1.0)]);
}

#[test]
fn test_grad_mixed() {
    let x = 1.0;
    let y = 2.0;
    let df = grad!(y * (x + y));
    assert_eq!(df.into_iter().filter(|(k, _v)| k == &x).collect::<Vec<_>>(), [(x, 2.0)]);
    // assert_eq!(df.into_iter().filter(|(k, _v)| k == &y).collect::<Vec<_>>(), [(y, 5.0)]);
}
