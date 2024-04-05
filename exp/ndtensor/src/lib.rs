extern crate acme;
#[cfg(not(feature = "std"))]
extern crate alloc;

#[allow(unused_imports)]
pub use self::utils::*;

#[macro_use]
pub(crate) mod seal;
#[macro_use]
pub(crate) mod utils;

pub mod data;
pub mod dim;
pub mod index;
pub mod iter;

pub mod prelude {
    #[doc(inline)]
    pub use crate::data::prelude::*;
    #[doc(inline)]
    pub use crate::iter::*;
}
