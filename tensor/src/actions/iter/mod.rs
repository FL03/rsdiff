/*
    Appellation: iter <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Iter
//!
//!
pub use self::{indexed::*, iterator::*, layout::*, utils::*};

#[allow(dead_code, unused)]
pub(crate) mod axis;
pub(crate) mod indexed;
pub(crate) mod iterator;
pub(crate) mod layout;

pub(crate) mod utils {
    use core::ptr;

    pub(crate) fn zip<I, J>(i: I, j: J) -> core::iter::Zip<I::IntoIter, J::IntoIter>
    where
        I: IntoIterator,
        J: IntoIterator,
    {
        i.into_iter().zip(j)
    }

    pub fn to_vec_mapped<I, F, B>(iter: I, mut f: F) -> Vec<B>
    where
        I: ExactSizeIterator, // + TrustedIterator
        F: FnMut(I::Item) -> B,
    {
        // Use an `unsafe` block to do this efficiently.
        // We know that iter will produce exactly .size() elements,
        // and the loop can vectorize if it's clean (without branch to grow the vector).
        let (size, _) = iter.size_hint();
        let mut result = Vec::with_capacity(size);
        let mut out_ptr = result.as_mut_ptr();
        let mut len = 0;
        iter.fold((), |(), elt| unsafe {
            ptr::write(out_ptr, f(elt));
            len += 1;
            result.set_len(len);
            out_ptr = out_ptr.offset(1);
        });
        debug_assert_eq!(size, result.len());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::shape::Layout;

    #[test]
    fn test_to_vec_mapped() {
        let v = [1, 2, 3, 4, 5];
        let f = |x| x * 2;
        let res = to_vec_mapped(v.iter(), f);
        assert_eq!(res, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_position() {
        let shape = (2, 2);
        let layout = Layout::contiguous(shape);
        let position = Position::new(1, vec![0, 1]);
        assert_eq!(position.index(), 1);
        assert_eq!(
            position.next(&layout).unwrap(),
            Position::new(2, vec![1, 0])
        );
    }
}
