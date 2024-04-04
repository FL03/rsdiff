/*
    Appellation: data <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Data
//!
//!
pub(crate) use self::utils::*;
pub use self::{container::*, layout::*, specs::*};

pub(crate) mod container;
pub(crate) mod layout;
pub(crate) mod specs;

pub mod elem;

pub mod repr {
    pub use self::{owned::OwnedRepr, shared::OwnedArcRepr, view::*};

    pub(crate) mod owned;
    pub(crate) mod shared;
    #[allow(dead_code)]
    pub(crate) mod view;
}

pub type Container<A = f64> = ContainerBase<repr::OwnedRepr<A>>;

pub type SharedContainer<A = f64> = ContainerBase<repr::OwnedArcRepr<A>>;

pub(crate) mod utils {
    use super::Layout;
    #[cfg(not(feature = "std"))]
    use alloc::vec::Vec;
    use core::ptr::NonNull;

    pub(crate) fn is_layout_c(layout: &Layout) -> bool {
        if let 1 = *layout.shape().rank() {
            return layout.stride[0] == 1 || layout.shape[0] <= 1;
        }

        for d in layout.shape().iter() {
            if *d == 0 {
                return true;
            }
        }

        let mut contig_stride = 1_isize;
        // check all dimensions -- a dimension of length 1 can have unequal strides
        for (dim, s) in izip!(layout.shape().iter().rev(), layout.stride().iter().rev()) {
            if *dim != 1 {
                let s = *s as isize;
                if s != contig_stride {
                    return false;
                }
                contig_stride *= *dim as isize;
            }
        }
        true
    }

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
    pub use super::layout::Layout;
    pub use super::repr::*;
    pub use super::specs::*;
}

#[cfg(test)]
mod tests {}
