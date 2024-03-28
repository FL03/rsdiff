/*
    Appellation: data <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Data
//!
//!
pub mod elem;

pub mod repr {
    pub mod owned;
}

pub trait Data: RawData {}

#[allow(clippy::missing_safety_doc)]
pub unsafe trait RawData {
    type Elem;

    #[doc(hidden)]
    fn _is_pointer_inbounds(&self, ptr: *const Self::Elem) -> bool;

    private_decl! {}
}

pub(crate) mod utils {
    #[cfg(not(feature = "std"))]
    #[allow(unused_imports)]
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

#[cfg(test)]
mod tests {}
