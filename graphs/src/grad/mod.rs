/*
    Appellation: grad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::id::GradientId;

pub(crate) mod id;

pub mod store;

pub(crate) mod prelude {
    pub use super::id::GradientId;
    pub use super::store::GradientStore;
}

#[cfg(test)]
mod tests {}
