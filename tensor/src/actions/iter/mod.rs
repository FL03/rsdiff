/*
    Appellation: iter <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Iter
//!
//!
pub use self::{indexed::*, iterator::*, position::*, utils::*};

#[allow(dead_code, unused)]
pub(crate) mod axis;
pub(crate) mod indexed;
pub(crate) mod iterator;
pub(crate) mod position;

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

}
