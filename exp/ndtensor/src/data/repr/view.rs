/*
    Appellation: view <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::data::specs::*;
use crate::data::{Container, ContainerBase, ContainerView, ContainerViewMut};
use crate::iter::{Baseiter, ElementsBase, ElementsBaseMut, Iter, IterMut};
use core::marker::PhantomData;
use core::ptr::NonNull;
use core::slice;

/// Array pointer’s representation.
///
/// *Don’t use this type directly—use the type aliases
/// [`RawArrayView`] / [`RawArrayViewMut`] for the array type!*
#[derive(Copy, Clone)]
// This is just a marker type, to carry the mutability and element type.
pub struct RawViewRepr<A> {
    ptr: PhantomData<A>,
}

impl<A> RawViewRepr<A> {
    #[inline(always)]
    const fn new() -> Self {
        RawViewRepr { ptr: PhantomData }
    }
}

/// Array view’s representation.
///
/// *Don’t use this type directly—use the type aliases
/// [`ArrayView`] / [`ArrayViewMut`] for the array type!*
#[derive(Copy, Clone)]
// This is just a marker type, to carry the lifetime parameter.
pub struct ViewRepr<A> {
    life: PhantomData<A>,
}

impl<A> ViewRepr<A> {
    #[inline(always)]
    const fn new() -> Self {
        ViewRepr { life: PhantomData }
    }
}

unsafe impl<'a, A> RawData for ViewRepr<&'a A> {
    type Elem = A;

    #[inline(always)]
    fn _is_pointer_inbounds(&self, _ptr: *const Self::Elem) -> bool {
        true
    }

    private_impl! {}
}

unsafe impl<'a, A> Data for ViewRepr<&'a A> {
    fn into_owned(self_: ContainerBase<Self>) -> Container<Self::Elem>
    where
        Self::Elem: Clone,
    {
        self_.to_owned()
    }

    fn try_into_owned_nocopy(
        self_: ContainerBase<Self>,
    ) -> Result<Container<Self::Elem>, ContainerBase<Self>> {
        Err(self_)
    }
}

unsafe impl<'a, A> RawDataClone for ViewRepr<&'a A> {
    unsafe fn clone_with_ptr(&self, ptr: NonNull<Self::Elem>) -> (Self, NonNull<Self::Elem>) {
        (*self, ptr)
    }
}

unsafe impl<'a, A> RawData for ViewRepr<&'a mut A> {
    type Elem = A;

    #[inline(always)]
    fn _is_pointer_inbounds(&self, _ptr: *const Self::Elem) -> bool {
        true
    }

    private_impl! {}
}

unsafe impl<'a, A> RawDataMut for ViewRepr<&'a mut A> {
    #[inline]
    fn try_ensure_unique(_: &mut ContainerBase<Self>)
    where
        Self: Sized,
    {
    }

    #[inline]
    fn try_is_unique(&mut self) -> Option<bool> {
        Some(true)
    }
}

unsafe impl<'a, A> Data for ViewRepr<&'a mut A> {
    fn into_owned(self_: ContainerBase<Self>) -> Container<Self::Elem>
    where
        Self::Elem: Clone,
    {
        self_.to_owned()
    }

    fn try_into_owned_nocopy(
        self_: ContainerBase<Self>,
    ) -> Result<Container<Self::Elem>, ContainerBase<Self>> {
        Err(self_)
    }
}

unsafe impl<'a, A> DataMut for ViewRepr<&'a mut A> {}

impl<'a, A> ContainerView<'a, A> {
    pub fn to_slice(&self) -> Option<&'a [A]> {
        if self.is_standard_layout() {
            unsafe { Some(slice::from_raw_parts(self.ptr.as_ptr(), self.size())) }
        } else {
            None
        }
    }
}

// Internal Methods
impl<'a, A> ContainerView<'a, A> {
    #[inline]
    pub(crate) fn into_base_iter(self) -> Baseiter<A> {
        unsafe {
            Baseiter::new(
                self.ptr.as_ptr(),
                self.shape().clone(),
                self.stride().clone(),
            )
        }
    }

    #[inline]
    pub(crate) fn into_elements_base(self) -> ElementsBase<'a, A> {
        ElementsBase::new(self)
    }

    pub(crate) fn into_iter_(self) -> Iter<'a, A> {
        Iter::new(self)
    }
}

impl<'a, A> ContainerViewMut<'a, A> {
    #[inline]
    pub(crate) fn into_base_iter(self) -> Baseiter<A> {
        unsafe {
            Baseiter::new(
                self.ptr.as_ptr(),
                self.shape().clone(),
                self.stride().clone(),
            )
        }
    }

    #[inline]
    pub(crate) fn into_elements_base(self) -> ElementsBaseMut<'a, A> {
        ElementsBaseMut::new(self)
    }

    pub(crate) fn into_iter_(self) -> IterMut<'a, A> {
        IterMut::new(self)
    }
    /// Return the array’s data as a slice, if it is contiguous and in standard order.
    /// Otherwise return self in the Err branch of the result.
    pub(crate) fn try_into_slice(self) -> Result<&'a mut [A], Self> {
        if self.is_standard_layout() {
            unsafe { Ok(slice::from_raw_parts_mut(self.ptr.as_ptr(), self.size())) }
        } else {
            Err(self)
        }
    }
}
