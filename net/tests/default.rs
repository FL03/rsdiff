#[cfg(test)]
mod tests {

    #[test]
    fn lib_compiles() {
        let f = |i: usize, j: usize| i * j;

        assert_eq!(f(10, 2), 20);
    }
}
