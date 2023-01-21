/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{apps::*, cli::*};

pub(crate) mod apps;
#[cfg(feature = "cli")]
pub(crate) mod cli;
