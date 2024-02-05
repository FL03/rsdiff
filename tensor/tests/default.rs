#[cfg(test)]

#[test]
fn compiles() {
    let add = |a, b| a + b;
    let result = add(2, 2);
    assert_eq!(result, 4);
}
