/*
    appellation: default <module>
    authors: @FL03
*/

#[test]
fn lib_compiles() {
    fn add<A, B, C>(lhs: A, rhs: B) -> C
    where
        A: core::ops::Add<B, Output = C>,
    {
        lhs + rhs
    }
    let result = f(2, 2);
    assert_eq!(result, 4);
}
