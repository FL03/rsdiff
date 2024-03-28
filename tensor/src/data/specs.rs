/*
    Appellation: specs <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::data::repr::OwnedArcRepr;
use crate::data::{ArcTensor, BaseTensor, Tensor};
use core::mem::MaybeUninit;
use core::ptr::NonNull;

/// Array representation trait.
///
/// For an array with elements that can be accessed with safe code.
///
/// ***Internal trait, see `RawData`.***
#[allow(clippy::missing_safety_doc)] // not implementable downstream
pub unsafe trait Data: RawData {
    /// Converts the array to a uniquely owned array, cloning elements if necessary.
    #[doc(hidden)]
    #[allow(clippy::wrong_self_convention)]
    fn into_owned(self_: BaseTensor<Self>) -> Tensor<Self::Elem>
    where
        Self::Elem: Clone;

    /// Converts the array into `Array<A, D>` if this is possible without
    /// cloning the array elements. Otherwise, returns `self_` unchanged.
    #[doc(hidden)]
    fn try_into_owned_nocopy<D>(
        self_: BaseTensor<Self>,
    ) -> Result<Tensor<Self::Elem>, BaseTensor<Self>>;

    /// Return a shared ownership (copy on write) array based on the existing one,
    /// cloning elements if necessary.
    #[doc(hidden)]
    #[allow(clippy::wrong_self_convention)]
    fn to_shared(self_: &BaseTensor<Self>) -> ArcTensor<Self::Elem>
    where
        Self::Elem: Clone;
}

#[allow(clippy::missing_safety_doc)] // not implementable downstream
pub unsafe trait DataMut: Data + RawDataMut {
    /// Ensures that the array has unique access to its data.
    #[doc(hidden)]
    #[inline]
    fn ensure_unique<D>(self_: &mut BaseTensor<Self>)
    where
        Self: Sized,
    {
        Self::try_ensure_unique(self_)
    }

    /// Returns whether the array has unique access to its data.
    #[doc(hidden)]
    #[inline]
    #[allow(clippy::wrong_self_convention)] // mut needed for Arc types
    fn is_unique(&mut self) -> bool {
        self.try_is_unique().unwrap()
    }
}

#[allow(clippy::missing_safety_doc)] // not implementable downstream
pub unsafe trait DataOwned: Data {
    /// Corresponding owned data with MaybeUninit elements
    type MaybeUninit: DataOwned<Elem = MaybeUninit<Self::Elem>>
        + RawDataSubst<Self::Elem, Output = Self>;
    #[doc(hidden)]
    fn new(elements: Vec<Self::Elem>) -> Self;

    /// Converts the data representation to a shared (copy on write)
    /// representation, without any copying.
    #[doc(hidden)]
    fn into_shared(self) -> OwnedArcRepr<Self::Elem>;
}

/// Array representation trait.
///
/// A representation that is a lightweight view.
///
/// ***Internal trait, see `Data`.***
#[allow(clippy::missing_safety_doc)] // not implementable downstream
pub unsafe trait DataShared: Clone + Data + RawDataClone {}

#[allow(clippy::missing_safety_doc)]
pub unsafe trait RawData: Sized {
    type Elem;

    #[doc(hidden)]
    fn _is_pointer_inbounds(&self, ptr: *const Self::Elem) -> bool;

    private_decl! {}
}

/// Array representation trait.
///
/// For an array with writable elements.
///
/// ***Internal trait, see `RawData`.***
#[allow(clippy::missing_safety_doc)] // not implementable downstream
pub unsafe trait RawDataMut: RawData {
    /// If possible, ensures that the array has unique access to its data.
    ///
    /// The implementer must ensure that if the input is contiguous, then the
    /// output has the same strides as input.
    ///
    /// Additionally, if `Self` provides safe mutable access to array elements,
    /// then this method **must** panic or ensure that the data is unique.
    #[doc(hidden)]
    fn try_ensure_unique(_: &mut BaseTensor<Self>)
    where
        Self: Sized;

    /// If possible, returns whether the array has unique access to its data.
    ///
    /// If `Self` provides safe mutable access to array elements, then it
    /// **must** return `Some(_)`.
    #[doc(hidden)]
    fn try_is_unique(&mut self) -> Option<bool>;
}

/// Array representation trait.
///
/// An array representation that can be cloned.
///
/// ***Internal trait, see `RawData`.***
#[allow(clippy::missing_safety_doc)] // not implementable downstream
pub unsafe trait RawDataClone: RawData {
    #[doc(hidden)]
    /// Unsafe because, `ptr` must point inside the current storage.
    unsafe fn clone_with_ptr(&self, ptr: NonNull<Self::Elem>) -> (Self, NonNull<Self::Elem>);

    #[doc(hidden)]
    unsafe fn clone_from_with_ptr(
        &mut self,
        other: &Self,
        ptr: NonNull<Self::Elem>,
    ) -> NonNull<Self::Elem> {
        let (data, ptr) = other.clone_with_ptr(ptr);
        *self = data;
        ptr
    }
}

pub trait RawDataSubst<A>: RawData {
    /// The resulting array storage of the same kind but substituted element type
    type Output: RawData<Elem = A>;

    /// Unsafely translate the data representation from one element
    /// representation to another.
    ///
    /// ## Safety
    ///
    /// Caller must ensure the two types have the same representation.
    unsafe fn data_subst(self) -> Self::Output;
}
