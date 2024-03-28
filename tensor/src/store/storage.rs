/*
    Appellation: storage <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{DataOwned, Layout, OwnedArcRepr, RawData, RawDataMut};
use core::ptr::NonNull;

pub type ArcStore<A = f64> = StoreBase<OwnedArcRepr<A>>;

#[derive(Clone)]
pub struct StoreBase<S>
where
    S: RawData,
{
    data: S,
    layout: Layout,
    ptr: NonNull<S::Elem>,
}

impl<A, S> StoreBase<S>
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
    pub fn into_shared(self) -> ArcStore<A>
    where
        S: DataOwned,
    {
        let data = self.data.into_shared();
        // safe because: equivalent unmoved data, ptr and dims remain valid
        // unsafe { Self::from_data_ptr(data, self.ptr).with_strides_dim(self.strides, self.dim) }
        unsafe { StoreBase::from_data_ptr(data, self.ptr) }
    }
    /// Return the number of elements in the tensor.
    pub fn size(&self) -> usize {
        self.layout.size()
    }
}

// Internal methods
impl<A, S> StoreBase<S>
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
    #[allow(dead_code)]
    pub(crate) unsafe fn with_layout(self, layout: Layout) -> Self {
        Self {
            data: self.data,
            layout,
            ptr: self.ptr,
        }
    }
}
