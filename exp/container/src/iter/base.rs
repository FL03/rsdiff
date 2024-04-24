/*
    Appellation: iter <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::data::{ContainerView, ContainerViewMut};
use acme::prelude::{Shape, Stride};
use core::marker::PhantomData;

#[derive(Clone, Debug)]
pub enum ElementsRepr<S, C> {
    Slice(S),
    Counted(C),
}

/// Counted read only iterator
#[derive(Debug)]
pub struct ElementsBase<'a, A> {
    inner: BaseIter<A>,
    _life: PhantomData<&'a A>,
}

impl<'a, A> ElementsBase<'a, A> {
    pub fn new(v: ContainerView<'a, A>) -> Self {
        ElementsBase {
            inner: v.into_base_iter(),
            _life: PhantomData,
        }
    }
}

impl<'a, A> Iterator for ElementsBase<'a, A> {
    type Item = &'a A;
    #[inline]
    fn next(&mut self) -> Option<&'a A> {
        self.inner.next().map(|p| unsafe { &*p })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn fold<Acc, G>(self, init: Acc, mut g: G) -> Acc
    where
        G: FnMut(Acc, Self::Item) -> Acc,
    {
        unsafe { self.inner.fold(init, move |acc, ptr| g(acc, &*ptr)) }
    }
}

impl<'a, A> DoubleEndedIterator for ElementsBase<'a, A> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a A> {
        self.inner.next_back().map(|p| unsafe { &*p })
    }

    fn rfold<Acc, G>(self, init: Acc, mut g: G) -> Acc
    where
        G: FnMut(Acc, Self::Item) -> Acc,
    {
        unsafe { self.inner.rfold(init, move |acc, ptr| g(acc, &*ptr)) }
    }
}

impl<'a, A> ExactSizeIterator for ElementsBase<'a, A> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// An iterator over the elements of an array.
///
/// Iterator element type is `&'a mut A`.
#[derive(Debug)]
pub struct ElementsBaseMut<'a, A> {
    inner: BaseIter<A>,
    life: PhantomData<&'a mut A>,
}

impl<'a, A> ElementsBaseMut<'a, A> {
    pub fn new(v: ContainerViewMut<'a, A>) -> Self {
        ElementsBaseMut {
            inner: v.into_base_iter(),
            life: PhantomData,
        }
    }
}

impl<'a, A> Iterator for ElementsBaseMut<'a, A> {
    type Item = &'a mut A;
    #[inline]
    fn next(&mut self) -> Option<&'a mut A> {
        self.inner.next().map(|p| unsafe { &mut *p })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn fold<Acc, G>(self, init: Acc, mut g: G) -> Acc
    where
        G: FnMut(Acc, Self::Item) -> Acc,
    {
        unsafe { self.inner.fold(init, move |acc, ptr| g(acc, &mut *ptr)) }
    }
}

impl<'a, A> DoubleEndedIterator for ElementsBaseMut<'a, A> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut A> {
        self.inner.next_back().map(|p| unsafe { &mut *p })
    }

    fn rfold<Acc, G>(self, init: Acc, mut g: G) -> Acc
    where
        G: FnMut(Acc, Self::Item) -> Acc,
    {
        unsafe { self.inner.rfold(init, move |acc, ptr| g(acc, &mut *ptr)) }
    }
}

impl<'a, A> ExactSizeIterator for ElementsBaseMut<'a, A> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Base for iterators over all axes.
///
/// Iterator element type is `*mut A`.
#[derive(Debug)]
pub struct BaseIter<A> {
    ptr: *mut A,
    shape: Shape,
    strides: Stride,
    index: Option<Vec<usize>>,
}

impl<A> BaseIter<A> {
    /// Creating a Baseiter is unsafe because shape and stride parameters need
    /// to be correct to avoid performing an unsafe pointer offset while
    /// iterating.
    #[inline]
    pub unsafe fn new(ptr: *mut A, shape: Shape, strides: Stride) -> BaseIter<A> {
        BaseIter {
            ptr,
            index: shape.first_index(),
            shape,
            strides,
        }
    }
}

impl<A> Iterator for BaseIter<A> {
    type Item = *mut A;

    #[inline]
    fn next(&mut self) -> Option<*mut A> {
        let index = match self.index {
            None => return None,
            Some(ref ix) => ix.clone(),
        };
        let offset = Shape::stride_offset(&index, &self.strides);
        self.index = self.shape.next_for(index);
        unsafe { Some(self.ptr.offset(offset)) }
    }
}

impl<A> ExactSizeIterator for BaseIter<A> {
    fn len(&self) -> usize {
        match self.index {
            None => 0,
            Some(ref ix) => {
                let gone = crate::default_strides(&self.shape)
                    .as_slice()
                    .iter()
                    .zip(ix.as_slice().iter())
                    .fold(0, |s, (&a, &b)| s + a * b);
                self.shape.size() - gone
            }
        }
    }
}

impl<A> DoubleEndedIterator for BaseIter<A> {
    #[inline]
    fn next_back(&mut self) -> Option<*mut A> {
        let index = match self.index.as_ref() {
            None => return None,
            Some(ix) => ix.clone(),
        };
        self.shape[0] -= 1;
        let offset = Shape::stride_offset(&self.shape, &self.strides);
        if index == self.shape {
            self.index = None;
        }

        unsafe { Some(self.ptr.offset(offset)) }
    }
}
