/*
    Appellation: props <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Properties
//!
//!

/// Associative Property describes an operation that is invariant under the grouping of its operands.
pub trait Associative {}

/// Commutative Property describes an operation that is invariant under the exchange of its operands.
pub trait Commutative {
    const IS_COMMUTATIVE: bool;
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_associative() {
        assert!(true);
    }

    #[test]
    fn test_communitative() {
        assert!(true);
    }
}
