#[cfg(test)]
#[test]
fn compiles() {
    let f = |l: usize, r: usize| l + r;
    let result = f(2, 2);
    assert_eq!(result, 4);
}
