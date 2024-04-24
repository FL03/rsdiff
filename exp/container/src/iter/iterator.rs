/*
    Appellation: iterator <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::base::*;
use crate::data::{ContainerView, ContainerViewMut};
use core::slice::{Iter as SliceIter, IterMut as SliceIterMut};
/// An iterator over the elements of a tensor.
///
/// Iterator element type is `&'a A`.
///
/// See [`.iter()`](ContainerBase::iter) for more information.

#[derive(Debug)]
pub struct Iter<'a, A> {
    inner: ElementsRepr<SliceIter<'a, A>, ElementsBase<'a, A>>,
}

impl<'a, A> Iter<'a, A> {
    pub(crate) fn new(self_: ContainerView<'a, A>) -> Self {
        Iter {
            inner: if let Some(slc) = self_.to_slice() {
                ElementsRepr::Slice(slc.iter())
            } else {
                ElementsRepr::Counted(self_.into_elements_base())
            },
        }
    }
}

impl<'a, A> Iterator for Iter<'a, A> {
    type Item = &'a A;
    #[inline]
    fn next(&mut self) -> Option<&'a A> {
        either_mut!(self.inner, iter => iter.next())
    }
}

#[derive(Debug)]
pub struct IterMut<'a, A> {
    inner: ElementsRepr<SliceIterMut<'a, A>, ElementsBaseMut<'a, A>>,
}

impl<'a, A> IterMut<'a, A> {
    pub(crate) fn new(self_: ContainerViewMut<'a, A>) -> Self {
        IterMut {
            inner: match self_.try_into_slice() {
                Ok(x) => ElementsRepr::Slice(x.iter_mut()),
                Err(self_) => ElementsRepr::Counted(self_.into_elements_base()),
            },
        }
    }
}
impl<'a, A> Iterator for IterMut<'a, A> {
    type Item = &'a mut A;
    #[inline]
    fn next(&mut self) -> Option<&'a mut A> {
        either_mut!(self.inner, iter => iter.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        either!(self.inner, ref iter => iter.size_hint())
    }

    fn fold<Acc, G>(self, init: Acc, g: G) -> Acc
    where
        G: FnMut(Acc, Self::Item) -> Acc,
    {
        either!(self.inner, iter => iter.fold(init, g))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        either_mut!(self.inner, iter => iter.nth(n))
    }

    fn collect<B>(self) -> B
    where
        B: FromIterator<Self::Item>,
    {
        either!(self.inner, iter => iter.collect())
    }

    fn all<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    {
        either_mut!(self.inner, iter => iter.all(f))
    }

    fn any<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    {
        either_mut!(self.inner, iter => iter.any(f))
    }

    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        either_mut!(self.inner, iter => iter.find(predicate))
    }

    fn find_map<B, F>(&mut self, f: F) -> Option<B>
    where
        F: FnMut(Self::Item) -> Option<B>,
    {
        either_mut!(self.inner, iter => iter.find_map(f))
    }

    fn count(self) -> usize {
        either!(self.inner, iter => iter.count())
    }

    fn last(self) -> Option<Self::Item> {
        either!(self.inner, iter => iter.last())
    }

    fn position<P>(&mut self, predicate: P) -> Option<usize>
    where
        P: FnMut(Self::Item) -> bool,
    {
        either_mut!(self.inner, iter => iter.position(predicate))
    }
}

impl<'a, A> DoubleEndedIterator for IterMut<'a, A> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut A> {
        either_mut!(self.inner, iter => iter.next_back())
    }

    fn nth_back(&mut self, n: usize) -> Option<&'a mut A> {
        either_mut!(self.inner, iter => iter.nth_back(n))
    }

    fn rfold<Acc, G>(self, init: Acc, g: G) -> Acc
    where
        G: FnMut(Acc, Self::Item) -> Acc,
    {
        either!(self.inner, iter => iter.rfold(init, g))
    }
}

impl<'a, A> ExactSizeIterator for IterMut<'a, A> {
    fn len(&self) -> usize {
        either!(self.inner, ref iter => iter.len())
    }
}
