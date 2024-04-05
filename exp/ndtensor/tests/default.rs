/*
    Appellation: default <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]

fn addition<A, B, C>(a: A, b: B) -> C
where
    A: std::ops::Add<B, Output = C>,
{
    a + b
}

#[test]
fn compiles() {
    let result = addition(2, 2);
    assert_eq!(result, 4);
}
