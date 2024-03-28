/*
    Appellation: data <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Data
//!
//!
pub use self::specs::*;

pub(crate) mod specs;

pub mod elem;

pub mod repr {
    pub use self::{owned::OwnedRepr, shared::OwnedArcRepr, view::*};

    pub(crate) mod owned;
    pub(crate) mod shared;
    pub(crate) mod view;
}

use crate::prelude::{BackpropOp, Layout, TensorId, TensorKind};
use core::ptr::NonNull;

pub type Tensor<A = f64> = BaseTensor<repr::OwnedRepr<A>>;

pub type ArcTensor<A = f64> = BaseTensor<repr::OwnedArcRepr<A>>;

#[derive(Clone)]
pub struct BaseTensor<S>
where
    S: RawData,
{
    id: TensorId,
    data: S,
    kind: TensorKind,
    layout: Layout,
    op: BackpropOp<S::Elem>,
    ptr: NonNull<S::Elem>,
}

impl<A, S> BaseTensor<S>
where
    S: RawData<Elem = A>,
{
    #[inline(always)]
    pub fn as_ptr(&self) -> *const A {
        self.ptr.as_ptr() as *const A
    }

    /// Return a mutable pointer to the first element in the array.
    ///
    /// This method attempts to unshare the data. If `S: DataMut`, then the
    /// data is guaranteed to be uniquely held on return.
    ///
    /// # Warning
    ///
    /// When accessing elements through this pointer, make sure to use strides
    /// obtained *after* calling this method, since the process of unsharing
    /// the data may change the strides.
    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut A
    where
        S: RawDataMut,
    {
        // self.try_ensure_unique(); // for ArcArray
        self.ptr.as_ptr()
    }

    /// Without any coping, turn the tensor into a shared tensor.
    pub fn into_shared(self) -> ArcTensor<A>
    where
        S: DataOwned,
    {
        let data = self.data.into_shared();
        // safe because: equivalent unmoved data, ptr and dims remain valid
        // unsafe { Self::from_data_ptr(data, self.ptr).with_strides_dim(self.strides, self.dim) }
        unsafe { BaseTensor::from_data_ptr(data, self.ptr) }
    }

    pub fn size(&self) -> usize {
        self.layout.size()
    }
}

// Internal methods
impl<A, S> BaseTensor<S>
where
    S: RawData<Elem = A>,
{
    pub(crate) unsafe fn from_data_ptr(data: S, ptr: NonNull<A>) -> Self {
        let tensor = Self {
            id: TensorId::new(),
            data,
            kind: TensorKind::Normal,
            layout: Layout::contiguous(0),
            op: BackpropOp::none(),
            ptr,
        };
        debug_assert!(tensor.pointer_is_inbounds());
        tensor
    }

    pub(crate) fn pointer_is_inbounds(&self) -> bool {
        self.data._is_pointer_inbounds(self.as_ptr())
    }

    pub(crate) unsafe fn with_layout(self, layout: Layout) -> BaseTensor<S> {
        Self {
            id: self.id,
            data: self.data,
            kind: self.kind,
            layout,
            op: self.op,
            ptr: self.ptr,
        }
    }
}

pub(crate) mod utils {
    #[cfg(not(feature = "std"))]
    use alloc::vec::Vec;
    use core::ptr::NonNull;

    /// Return a NonNull<T> pointer to the vector's data
    pub(crate) fn nonnull_from_vec_data<T>(v: &mut Vec<T>) -> NonNull<T> {
        // this pointer is guaranteed to be non-null
        unsafe { NonNull::new_unchecked(v.as_mut_ptr()) }
    }

    /// Converts `ptr` to `NonNull<T>`
    ///
    /// Safety: `ptr` *must* be non-null.
    /// This is checked with a debug assertion, and will panic if this is not true,
    /// but treat this as an unconditional conversion.
    #[allow(dead_code)]
    #[inline]
    pub(crate) unsafe fn nonnull_debug_checked_from_ptr<T>(ptr: *mut T) -> NonNull<T> {
        debug_assert!(!ptr.is_null());
        NonNull::new_unchecked(ptr)
    }
}

pub(crate) mod prelude {
    pub use super::repr::*;
    pub use super::specs::*;
}

#[cfg(test)]
mod tests {}
