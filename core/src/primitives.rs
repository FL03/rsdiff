/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::types::*;

mod constants {}

mod statics {}

mod types {
    /// A boxed error type for use in the library.
    pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
    /// A boxed result type for use in the library.
    pub type BoxResult<T = ()> = std::result::Result<T, BoxError>;
}
