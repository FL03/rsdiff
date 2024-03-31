/*
    Appellation: data <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Data
//!
//!
pub(crate) use self::utils::*;
pub use self::{specs::*, tensor::*};

pub(crate) mod specs;
pub(crate) mod tensor;

pub mod elem;

pub mod repr {
    pub use self::{owned::OwnedRepr, shared::OwnedArcRepr, view::*};

    pub(crate) mod owned;
    pub(crate) mod shared;
    pub(crate) mod view;
}

pub type Tensor<A = f64> = BaseTensor<repr::OwnedRepr<A>>;

pub type ArcTensor<A = f64> = BaseTensor<repr::OwnedArcRepr<A>>;

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
