/*
    Appellation: container <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::specs::{Data, DataOwned, RawData, RawDataMut};
use super::{nonnull_from_vec_data, Container, SharedContainer};
use crate::actions::iter::to_vec_mapped;
use crate::prelude::Layout;
use crate::shape::dim::can_index_slice;
use crate::shape::{IntoShape, IntoStride, Shape, Stride};
use core::ptr::NonNull;
use core::slice;
use rawpointer::PointerExt;

#[derive(Clone)]
pub struct ContainerBase<S>
where
    S: RawData,
{
    pub(crate) data: S,
    pub(crate) layout: Layout,
    pub(crate) ptr: NonNull<S::Elem>,
}

impl<A, S> ContainerBase<S>
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
        RawDataMut::try_ensure_unique(self); // for ArcArray
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
    /// Without any coping, turn the tensor into a shared tensor.
    pub fn into_shared(self) -> SharedContainer<A>
    where
        S: DataOwned,
    {
        let data = self.data.into_shared();
        // safe because: equivalent unmoved data, ptr and dims remain valid
        unsafe { ContainerBase::from_data_ptr(data, self.ptr).with_layout(self.layout) }
    }
    /// Return true if the array is known to be contiguous.
    pub fn is_contiguous(&self) -> bool {
        self.layout().is_contiguous()
    }
    /// Return true if the array is known to be c-contiguous (Row Major)
    pub fn is_standard_layout(&self) -> bool {
        self.layout().is_layout_c()
    }
    ///
    pub fn iter(&self) -> slice::Iter<'_, A>
    where
        S: Data,
    {
        dbg!("Implement a custom iter for ContainerBase");
        self.as_slice_memory_order().unwrap().iter()
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn map<'a, B, F>(&'a self, f: F) -> Container<B>
    where
        F: FnMut(&'a A) -> B,
        A: 'a,
        S: Data,
    {
        unsafe {
            if let Some(slc) = self.as_slice_memory_order() {
                ContainerBase::from_shape_trusted_iter_unchecked(
                    self.shape().slice(),
                    slc.iter(),
                    f,
                )
            } else {
                ContainerBase::from_shape_trusted_iter_unchecked(self.shape(), self.iter(), f)
            }
        }
    }

    pub fn mapv<B, F>(&self, mut f: F) -> Container<B>
    where
        F: FnMut(A) -> B,
        A: Clone,
        S: Data,
    {
        self.map(move |x| f(x.clone()))
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
impl<A, S> ContainerBase<S>
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
        ContainerBase::from_data_ptr(DataOwned::new(v), ptr).with_layout(layout)
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

impl<A, S> ContainerBase<S>
where
    S: RawData<Elem = A>,
{
    pub(crate) unsafe fn from_data_ptr(data: S, ptr: NonNull<A>) -> Self {
        let tensor = Self {
            data,
            layout: Layout::contiguous(0),
            ptr,
        };
        debug_assert!(tensor.pointer_is_inbounds());
        tensor
    }

    pub(crate) fn pointer_is_inbounds(&self) -> bool {
        self.data._is_pointer_inbounds(self.as_ptr())
    }

    pub(crate) unsafe fn with_layout(self, layout: Layout) -> ContainerBase<S> {
        debug_assert_eq!(self.layout().rank(), layout.rank());

        Self {
            data: self.data,
            layout,
            ptr: self.ptr,
        }
    }
}
