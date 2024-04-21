/*
    Appellation: fields <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// A field is a set of elements with two binary operations, addition and multiplication, that satisfy the following axioms:
/// - Associativity of addition and multiplication
/// - Commutativity of addition and multiplication
pub trait Field {
    type Elem: ?Sized;

    /// The length of the field.
    fn len(&self) -> usize;
    /// The rank of the field; i.e the number of dimensions.
    fn rank(&self) -> usize;
}

pub trait Scalar {
    type Complex: Scalar;
    type Real: Scalar;
}

#[cfg(test)]
mod tests {}
