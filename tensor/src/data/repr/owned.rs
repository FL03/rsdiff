/*
    Appellation: owned <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::data::utils::nonnull_from_vec_data;
use crate::data::RawData;
use core::mem::{self, ManuallyDrop};
use core::ptr::NonNull;
use core::slice;

#[derive(Debug)]
#[repr(C)]
pub struct OwnedRepr<A> {
    capacity: usize,
    len: usize,
    ptr: NonNull<A>,
}

impl<A> OwnedRepr<A> {
    pub fn from_vec(vec: Vec<A>) -> Self {
        let mut v = ManuallyDrop::new(vec);
        let capacity = v.capacity();
        let len = v.len();
        let ptr = nonnull_from_vec_data(&mut v);

        Self { capacity, len, ptr }
    }

    pub(crate) fn as_slice(&self) -> &[A] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn ptr(&self) -> NonNull<A> {
        self.ptr
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

    fn take_as_vec(&mut self) -> Vec<A> {
        let capacity = self.capacity;
        let len = self.len;

        self.capacity = 0;
        self.len = 0;

        unsafe { Vec::from_raw_parts(self.ptr.as_ptr(), len, capacity) }
    }

    pub(crate) fn into_vec(self) -> Vec<A> {
        ManuallyDrop::new(self).take_as_vec()
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

unsafe impl<A> Send for OwnedRepr<A> {}

unsafe impl<A> Sync for OwnedRepr<A> {}

impl<A> From<Vec<A>> for OwnedRepr<A> {
    fn from(vec: Vec<A>) -> Self {
        Self::from_vec(vec)
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
