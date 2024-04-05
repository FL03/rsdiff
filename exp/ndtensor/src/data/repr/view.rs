/*
    Appellation: view <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::marker::PhantomData;

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
