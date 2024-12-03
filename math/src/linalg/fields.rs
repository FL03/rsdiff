/*
    Appellation: fields <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::marker::PhantomData;
use num::complex::Complex;
use num::traits::real::Real;
/// A field is a set of elements with two binary operations, addition and multiplication, that satisfy the following axioms:
/// - Associativity of addition and multiplication
/// - Commutativity of addition and multiplication
pub trait Field {
    const RANK: Option<usize> = None;
    type Elem: ?Sized;

    fn kind(&self) -> Fields;
    /// The rank of the field; i.e the number of dimensions.
    fn rank(&self) -> usize;
    /// The number of elements in the field.
    fn size(&self) -> usize;
}

pub enum Fields {
    Real(R),
    Complex(C),
}

pub struct R<T = f64>
where
    T: Real,
{
    _elem: PhantomData<T>,
}

pub struct C<T = f64>
where
    T: Real,
{
    _elem: PhantomData<Complex<T>>,
}

pub trait ComplexField {
    type DType;
}

pub trait Scalar {
    type Complex: ComplexField<DType = Self::Real>;
    type Real: Scalar<Real = Self::Real>;
}

#[cfg(test)]
mod tests {}
