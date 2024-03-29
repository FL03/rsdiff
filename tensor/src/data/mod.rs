/*
    Appellation: data <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Data
//!
//!
pub use self::specs::*;
pub(crate) use self::utils::*;

pub(crate) mod specs;

pub mod elem;

pub mod repr {
    pub use self::{owned::OwnedRepr, shared::OwnedArcRepr, view::*};

    pub(crate) mod owned;
    pub(crate) mod shared;
    pub(crate) mod view;
}

use crate::actions::iter::to_vec_mapped;
use crate::prelude::{BackpropOp, Layout, TensorId, TensorKind};
use crate::shape::dim::can_index_slice;
use crate::shape::{IntoShape, IntoStride, Shape, Stride};
use core::ptr::NonNull;
use core::slice;
use rawpointer::PointerExt;

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
    pub fn as_slice_memory_order(&self) -> Option<&[A]>
    where
        S: Data,
    {
        if self.is_contiguous() {
            let offset = self.layout.offset_from_low_addr_ptr_to_logical_ptr();
            unsafe {
                Some(slice::from_raw_parts(
                    PointerExt::sub(self.ptr, offset).as_ptr(),
                    self.size(),
                ))
            }
        } else {
            None
        }
    }

    /// Return true if the array is known to be contiguous.
    pub fn is_contiguous(&self) -> bool {
        self.layout.is_contiguous()
    }

    pub fn is_standard_layout(&self) -> bool {
        self.layout.is_layout_c()
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

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn map<'a, B, F>(&'a self, f: F) -> Tensor<B>
    where
        F: FnMut(&'a A) -> B,
        A: 'a,
        S: Data,
    {
        unsafe {
            if let Some(slc) = self.as_slice_memory_order() {
                BaseTensor::from_shape_trusted_iter_unchecked(self.shape().slice(), slc.iter(), f)
            } else {
                unimplemented!()
                // BaseTensor::from_shape_trusted_iter_unchecked(self.shape(), self.iter(), f)
            }
        }
    }

    pub fn shape(&self) -> &Shape {
        self.layout().shape()
    }

    pub fn stride(&self) -> &Stride {
        self.layout().stride()
    }

    pub fn size(&self) -> usize {
        self.layout.size()
    }
}

// Internal methods
impl<A, S> BaseTensor<S>
where
    S: DataOwned + RawData<Elem = A>,
{
    unsafe fn from_vec_dim_stride_unchecked(
        dim: impl IntoShape,
        strides: impl IntoStride,
        mut v: Vec<A>,
    ) -> Self {
        let layout = Layout::new(0, dim, strides);
        // debug check for issues that indicates wrong use of this constructor
        debug_assert!(can_index_slice(&v, &layout.shape(), &layout.stride()).is_ok());

        let ptr = {
            let tmp = nonnull_from_vec_data(&mut v);
            PointerExt::add(tmp, layout.offset_from_low_addr_ptr_to_logical_ptr())
        };
        BaseTensor::from_data_ptr(DataOwned::new(v), ptr).with_layout(layout)
    }

    /// Creates an array from an iterator, mapped by `map` and interpret it according to the
    /// provided shape and strides.
    ///
    /// # Safety
    ///
    /// See from_shape_vec_unchecked
    pub(crate) unsafe fn from_shape_trusted_iter_unchecked<Sh, I, F>(
        shape: Sh,
        iter: I,
        map: F,
    ) -> Self
    where
        Sh: IntoShape,
        I: ExactSizeIterator,
        F: FnMut(I::Item) -> A,
    {
        let shape = shape.into_shape();
        let strides = shape.default_strides(); // shape.stride().strides_for_dim(&dim);
        let v = to_vec_mapped(iter, map);
        Self::from_vec_dim_stride_unchecked(shape, strides, v)
    }
}

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

    pub(crate) unsafe fn with_strides_dim(
        self,
        stride: impl IntoStride,
        dim: impl IntoShape,
    ) -> BaseTensor<S> {
        let shape = dim.into_shape();
        let stride = stride.into_stride();
        debug_assert_eq!(shape.rank(), stride.rank());

        let layout = Layout::new(0, shape, stride);
        self.with_layout(layout)
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
