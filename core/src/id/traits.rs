/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::borrow::Borrow;

/// An `Identifier` is a type that can be used as an identifier
pub trait Identifier: ToString {
    private!();
}

/// The `Id` trait describes the behavior of a type that can be used as an id.
/// An `Id` is almost identical to an `Identifier`, but it is a trait that can be implemented for any type.
///
pub trait Id<K>
where
    K: Identifier,
{
    type Q: Borrow<K>;

    fn from_id(id: Self::Q) -> Self;

    fn get(&self) -> &Self::Q;
}

pub trait Identifiable: Identify {
    fn get_id(&self) -> &Self::Id;
}

pub trait IdentifierExt: Identifier
where
    Self: Copy + Eq + Ord + core::hash::Hash,
{
}

pub trait HashId: Identifier
where
    Self: Eq + core::hash::Hash,
{
}

pub trait IntoId {
    type Id: Identifier;

    fn into_id(self) -> Self::Id;
}

pub trait Identify {
    type Id: Identifier;

    fn id(&self) -> &Self::Id;
}

pub trait IdentifyMut: Identify {
    fn id_mut(&mut self) -> &mut Self::Id;
}

/*
 *********** impls ***********
*/
impl<K, Q> Id<K> for Q
where
    K: Identifier,
    Q: Borrow<K>,
{
    type Q = Q;

    fn from_id(id: Self::Q) -> Self {
        id
    }

    fn get(&self) -> &Self::Q {
        self
    }
}

impl<S> Identify for S
where
    S: Identifier,
{
    type Id = S;

    fn id(&self) -> &Self::Id {
        self
    }
}

impl<Id> HashId for Id where Id: Eq + Identifier + core::hash::Hash {}

impl<Id> IdentifierExt for Id where Id: Copy + Eq + Identifier + Ord + core::hash::Hash {}

macro_rules! identifier {
    ($($t:ty),*) => {
        $(
            identifier!(@impl $t);
        )*
    };
    (@impl $t:ty) => {
        impl Identifier for $t {
            seal!();
        }
    };
}

identifier!(f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
identifier!(bool, char, &str, String);
