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
    use crate::create::Linspace;
    use crate::prelude::{IntoShape, Layout, Shape, Tensor};
    use num::traits::{FromPrimitive, Num};

    fn linvec<T>(n: usize) -> (Vec<T>, usize)
    where
        T: Copy + Default + FromPrimitive + Num + PartialOrd,
    {
        let space = Vec::linspace(T::zero(), T::from_usize(n).unwrap(), n);
        (space, n)
    }

    #[test]
    fn test_layout_iter() {
        let shape = (2, 2).into_shape();
        let layout = Layout::contiguous(shape);
        let exp = [vec![0usize, 0], vec![0, 1], vec![1, 0], vec![1, 1]];
        for (pos, exp) in layout.iter().zip(exp.iter()) {
            assert_eq!(pos.position(), *exp);
        }
        for (pos, exp) in layout.iter().rev().zip(exp.iter().rev()) {
            assert_eq!(pos.position(), *exp);
        }
    }

    #[test]
    fn test_iter() {
        let shape = Shape::from_iter([2, 2, 2, 2]);
        let (exp, n) = linvec::<f64>(shape.size());
        let tensor = Tensor::linspace(0f64, n as f64, n)
            .reshape(shape.clone())
            .unwrap();
        assert_eq!(&tensor, &exp);

        let mut tensor = Tensor::zeros(shape);
        for (elem, val) in tensor.iter_mut().zip(exp.iter()) {
            *elem = *val;
        }
        assert_eq!(&tensor, &exp);
    }

    #[test]
    fn test_iter_mut_rev() {
        let shape = Shape::from_iter([2, 2, 2, 2]);
        let n = shape.size();
        let exp = Vec::linspace(0f64, n as f64, n);
        let rev = exp.iter().rev().copied().collect::<Vec<f64>>();
        let mut tensor = Tensor::zeros(shape);
        for (elem, val) in tensor.iter_mut().rev().zip(exp.iter()) {
            *elem = *val;
        }
        assert_eq!(&tensor, &rev);
    }

    #[test]
    fn test_iter_rev() {
        let shape = Shape::from_iter([2, 2]);
        let n = shape.size();
        let exp = Vec::linspace(0f64, n as f64, n);
        let tensor = Tensor::linspace(0f64, n as f64, n).reshape(shape).unwrap();

        for (i, j) in tensor.iter().rev().zip(exp.iter().rev()) {
            assert_eq!(i, j);
        }
    }
}
