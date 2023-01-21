#[cfg(test)]
#[test]
fn lib_compiles() {
    let f = |l: usize, r: usize| l + r;
    let result = f(2, 2);
    assert_eq!(result, 4);
}
