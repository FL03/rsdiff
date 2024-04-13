/*
    Appellation: vs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::math::linalg::fields::Field;

pub trait VectorSpace<T> {
    type Field: Field<Elem = T>;

    fn field(&self) -> &Self::Field;

    fn rank(&self) -> usize {
        self.field().rank()
    }
}
