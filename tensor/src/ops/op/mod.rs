/*
    Appellation: ops <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod binary;
pub mod unary;

#[cfg(test)]
mod tests {

    #[test]
    fn test_ops() {
        assert_eq!(1 + 1, 2);
    }
}
