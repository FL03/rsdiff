/*
    Appellation: owned <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::data::repr::OwnedArcRepr;
use crate::data::utils::nonnull_from_vec_data;
use crate::data::{Container, ContainerBase, SharedContainer};
use crate::data::{Data, DataMut, DataOwned, RawData, RawDataClone, RawDataMut, RawDataSubst};
use core::mem::{self, ManuallyDrop, MaybeUninit};
use core::ptr::NonNull;
use core::slice;
use rawpointer::PointerExt;
use std::sync::Arc;

#[derive(Debug)]
#[repr(C)]
pub struct OwnedRepr<A> {
    capacity: usize,
    len: usize,
    ptr: NonNull<A>,
}

impl<A> OwnedRepr<A> {
    /// Create an [OwnedRepr] from a [Vec]
    pub fn from_vec(vec: Vec<A>) -> Self {
        let mut v = ManuallyDrop::new(vec);

        Self {
            capacity: v.capacity(),
            len: v.len(),
            ptr: nonnull_from_vec_data(&mut v),
        }
    }

    pub fn as_ptr(&self) -> *const A {
        self.ptr.as_ptr()
    }

    pub fn as_ptr_mut(&mut self) -> *mut A {
        self.ptr.as_ptr()
    }

    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    pub const fn len(&self) -> usize {
        self.len
    }
}

// Internal methods
#[allow(dead_code)]
impl<A> OwnedRepr<A> {
    pub(crate) fn as_nonnull_mut(&mut self) -> NonNull<A> {
        self.ptr
    }

    pub(crate) fn as_slice(&self) -> &[A] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    /// Cast self into equivalent repr of other element type
    ///
    /// ## Safety
    ///
    /// Caller must ensure the two types have the same representation.
    /// **Panics** if sizes don't match (which is not a sufficient check).
    pub(crate) unsafe fn data_subst<B>(self) -> OwnedRepr<B> {
        // necessary but not sufficient check
        assert_eq!(mem::size_of::<A>(), mem::size_of::<B>());
        let self_ = ManuallyDrop::new(self);
        OwnedRepr {
            ptr: self_.ptr.cast::<B>(),
            len: self_.len,
            capacity: self_.capacity,
        }
    }

    pub(crate) fn into_vec(self) -> Vec<A> {
        ManuallyDrop::new(self).take_as_vec()
    }
    /// Set the valid length of the data
    ///
    /// ## Safety
    ///
    /// The first `new_len` elements of the data should be valid.
    pub(crate) unsafe fn set_len(&mut self, new_len: usize) {
        debug_assert!(new_len <= self.capacity);
        self.len = new_len;
    }

    pub(crate) fn take_as_vec(&mut self) -> Vec<A> {
        let capacity = self.capacity;
        let len = self.len;

        self.capacity = 0;
        self.len = 0;

        unsafe { Vec::from_raw_parts(self.ptr.as_ptr(), len, capacity) }
    }
}

impl<A> Clone for OwnedRepr<A>
where
    A: Clone,
{
    fn clone(&self) -> Self {
        Self::from(self.as_slice().to_owned())
    }

    fn clone_from(&mut self, other: &Self) {
        let mut v = self.take_as_vec();
        let other = other.as_slice();

        if v.len() > other.len() {
            v.truncate(other.len());
        }
        let (front, back) = other.split_at(v.len());
        v.clone_from_slice(front);
        v.extend_from_slice(back);
        *self = Self::from(v);
    }
}

impl<A> Drop for OwnedRepr<A> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            // correct because: If the elements don't need dropping, an
            // empty Vec is ok. Only the Vec's allocation needs dropping.
            //
            // implemented because: in some places in ndarray
            // where A: Copy (hence does not need drop) we use uninitialized elements in
            // vectors. Setting the length to 0 avoids that the vector tries to
            // drop, slice or otherwise produce values of these elements.
            // (The details of the validity letting this happen with nonzero len, are
            // under discussion as of this writing.)
            if !mem::needs_drop::<A>() {
                self.len = 0;
            }
            // drop as a Vec.
            self.take_as_vec();
        }
    }
}

unsafe impl<A> Data for OwnedRepr<A> {
    #[inline]
    fn into_owned(self_: ContainerBase<Self>) -> Container<Self::Elem>
    where
        A: Clone,
    {
        self_
    }

    #[inline]
    fn try_into_owned_nocopy(
        self_: ContainerBase<Self>,
    ) -> Result<Container<Self::Elem>, ContainerBase<Self>> {
        Ok(self_)
    }

    fn to_shared(self_: &ContainerBase<Self>) -> SharedContainer<Self::Elem>
    where
        Self::Elem: Clone,
    {
        // clone to shared
        self_.to_owned().into_shared()
    }
}

unsafe impl<A> DataMut for OwnedRepr<A> {}

unsafe impl<A> DataOwned for OwnedRepr<A> {
    type MaybeUninit = OwnedRepr<MaybeUninit<A>>;

    fn new(elements: Vec<A>) -> Self {
        OwnedRepr::from(elements)
    }

    fn into_shared(self) -> OwnedArcRepr<A> {
        OwnedArcRepr(Arc::new(self))
    }
}

unsafe impl<A> RawData for OwnedRepr<A> {
    type Elem = A;

    fn _is_pointer_inbounds(&self, self_ptr: *const Self::Elem) -> bool {
        let slc = self.as_slice();
        let ptr = slc.as_ptr() as *mut A;
        let end = unsafe { ptr.add(slc.len()) };
        self_ptr >= ptr && self_ptr <= end
    }

    private_impl! {}
}

unsafe impl<A> RawDataMut for OwnedRepr<A> {
    fn try_ensure_unique(_: &mut ContainerBase<Self>)
    where
        Self: Sized,
    {
    }

    fn try_is_unique(&mut self) -> Option<bool> {
        Some(true)
    }
}

unsafe impl<A> RawDataClone for OwnedRepr<A>
where
    A: Clone,
{
    unsafe fn clone_with_ptr(&self, ptr: NonNull<Self::Elem>) -> (Self, NonNull<Self::Elem>) {
        let mut u = self.clone();
        let mut new_ptr = u.as_nonnull_mut();
        if mem::size_of::<A>() != 0 {
            let our_off =
                (ptr.as_ptr() as isize - self.as_ptr() as isize) / mem::size_of::<A>() as isize;
            new_ptr = PointerExt::offset(new_ptr, our_off);
        }
        (u, new_ptr)
    }

    unsafe fn clone_from_with_ptr(
        &mut self,
        other: &Self,
        ptr: NonNull<Self::Elem>,
    ) -> NonNull<Self::Elem> {
        let our_off = if mem::size_of::<A>() != 0 {
            (ptr.as_ptr() as isize - other.as_ptr() as isize) / mem::size_of::<A>() as isize
        } else {
            0
        };
        self.clone_from(other);
        PointerExt::offset(self.as_nonnull_mut(), our_off)
    }
}

impl<A, B> RawDataSubst<B> for OwnedRepr<A> {
    type Output = OwnedRepr<B>;

    unsafe fn data_subst(self) -> Self::Output {
        self.data_subst()
    }
}

unsafe impl<A> Send for OwnedRepr<A> {}

unsafe impl<A> Sync for OwnedRepr<A> {}

impl<A> From<Vec<A>> for OwnedRepr<A> {
    fn from(vec: Vec<A>) -> Self {
        Self::from_vec(vec)
    }
}
