/*
    Appellation: acme
    Context: Library
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
 */
pub(crate) mod actors;
pub(crate) mod chains;
pub(crate) mod controllers;
pub(crate) mod utils;

pub use crate::{
    actors::*,
    chains::*,
    common::*,
    controllers::*,
    errors::*,
    utils::*
};


#[doc(inline)]
#[cfg(feature = "core")]
pub use acme_core::*;
#[doc(inline)]
#[cfg(feature = "macros")]
pub use acme_macros::*;
#[doc(inline)]
#[cfg(feature = "network")]
pub use acme_network::*;

pub(crate) mod common {
    pub use bson::DateTime as TimeStamp;
    pub use bson::oid::ObjectId;
    pub use chrono::Local as LocalTime;

    pub const DIFFICULTY_PREFIX: &str = "00";

    pub type BlockData = String;
    pub type BlockId = ObjectId;
    pub type BlockHash = String;
    pub type BlockNonce = u64;

    pub type Container<T = String> = Dictionary<Vec<T>>;
    pub type Dictionary<T = String> = std::collections::HashMap<String, T>;

    pub enum Dates {
        Datetime(chrono::DateTime<chrono::Local>),
        Localtime(chrono::Local),
        Timestamp(bson::DateTime),
    }

    pub type DateTime = chrono::DateTime<LocalTime>;
}

pub(crate) mod errors {
    use std::error::Error;

    pub use config::ConfigError;

    pub enum Errors {
        AsyncError,
        BoxedError,
    }

    pub type AsyncError = Box<dyn Error + Send + Sync + 'static>;
    pub type BoxedError = Box<dyn Error>;
}

