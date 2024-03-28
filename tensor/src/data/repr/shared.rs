/*
    Appellation: shared <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::data::repr::OwnedRepr;
use crate::data::specs::*;
use crate::data::{ArcTensor, BaseTensor, Tensor};
#[cfg(not(feature = "std"))]
use alloc::sync::Arc;
use core::mem::MaybeUninit;
use core::ptr::NonNull;
use rawpointer::PointerExt;
#[cfg(feature = "std")]
use std::sync::Arc;

#[derive(Debug)]
pub struct OwnedArcRepr<A>(pub(crate) Arc<OwnedRepr<A>>);

impl<A> Clone for OwnedArcRepr<A> {
    fn clone(&self) -> Self {
        OwnedArcRepr(self.0.clone())
    }
}

unsafe impl<A> Data for OwnedArcRepr<A> {
    fn into_owned(self_: BaseTensor<Self>) -> crate::data::Tensor<Self::Elem>
    where
        Self::Elem: Clone,
    {
        // Self::ensure_unique(&mut self_);
        let data = Arc::try_unwrap(self_.data.0).ok().unwrap();
        // safe because data is equivalent
        unsafe { BaseTensor::from_data_ptr(data, self_.ptr).with_layout(self_.layout) }
    }

    fn try_into_owned_nocopy<D>(
        self_: BaseTensor<Self>,
    ) -> Result<Tensor<Self::Elem>, BaseTensor<Self>> {
        match Arc::try_unwrap(self_.data.0) {
            Ok(owned_data) => unsafe {
                // Safe because the data is equivalent.
                Ok(BaseTensor::from_data_ptr(owned_data, self_.ptr).with_layout(self_.layout))
            },
            Err(arc_data) => unsafe {
                // Safe because the data is equivalent; we're just
                // reconstructing `self_`.
                Err(BaseTensor::from_data_ptr(OwnedArcRepr(arc_data), self_.ptr)
                    .with_layout(self_.layout))
            },
        }
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_shared(self_: &BaseTensor<Self>) -> ArcTensor<Self::Elem>
    where
        Self::Elem: Clone,
    {
        // to shared using clone of OwnedArcRepr without clone of raw data.
        self_.clone()
    }
}

unsafe impl<A> DataMut for OwnedArcRepr<A> where A: Clone {}

unsafe impl<A> DataOwned for OwnedArcRepr<A> {
    type MaybeUninit = OwnedArcRepr<MaybeUninit<A>>;

    fn new(elements: Vec<A>) -> Self {
        OwnedArcRepr(Arc::new(OwnedRepr::from(elements)))
    }

    fn into_shared(self) -> OwnedArcRepr<A> {
        self
    }
}

unsafe impl<A> DataShared for OwnedArcRepr<A> {}

unsafe impl<A> RawData for OwnedArcRepr<A> {
    type Elem = A;

    fn _is_pointer_inbounds(&self, self_ptr: *const Self::Elem) -> bool {
        self.0._is_pointer_inbounds(self_ptr)
    }

    private_impl! {}
}

unsafe impl<A> RawDataClone for OwnedArcRepr<A> {
    unsafe fn clone_with_ptr(&self, ptr: NonNull<Self::Elem>) -> (Self, NonNull<Self::Elem>) {
        // pointer is preserved
        (self.clone(), ptr)
    }
}

// NOTE: Copy on write
unsafe impl<A> RawDataMut for OwnedArcRepr<A>
where
    A: Clone,
{
    fn try_ensure_unique(self_: &mut BaseTensor<Self>)
    where
        Self: Sized,
    {
        if Arc::get_mut(&mut self_.data.0).is_some() {
            return;
        }
        if self_.size() <= self_.data.0.len() / 2 {
            // Clone only the visible elements if the current view is less than
            // half of backing data.
            *self_ = self_.to_owned().into_shared();
            return;
        }
        let rcvec = &mut self_.data.0;
        let a_size = core::mem::size_of::<A>() as isize;
        let our_off = if a_size != 0 {
            (self_.ptr.as_ptr() as isize - rcvec.as_ptr() as isize) / a_size
        } else {
            0
        };
        let rvec = Arc::make_mut(rcvec);
        unsafe {
            self_.ptr = PointerExt::offset(rvec.as_nonnull_mut(), our_off);
        }
    }

    fn try_is_unique(&mut self) -> Option<bool> {
        Some(Arc::get_mut(&mut self.0).is_some())
    }
}

impl<A, B> RawDataSubst<B> for OwnedArcRepr<A> {
    type Output = OwnedArcRepr<B>;

    unsafe fn data_subst(self) -> Self::Output {
        OwnedArcRepr(Arc::from_raw(Arc::into_raw(self.0) as *const OwnedRepr<B>))
    }
}
