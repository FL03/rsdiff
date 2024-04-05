/*
    Appellation: error <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(Clone, Debug, Display, EnumCount, EnumIs, VariantNames)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case", untagged)
)]
#[repr(usize)]
#[strum(serialize_all = "snake_case")]
pub enum GraphError {
    Cycle(CycleError),
    Unknown(String),
}

unsafe impl Send for GraphError {}

unsafe impl Sync for GraphError {}

impl std::error::Error for GraphError {}

impl From<&str> for GraphError {
    fn from(error: &str) -> Self {
        GraphError::Unknown(error.to_string())
    }
}

impl From<String> for GraphError {
    fn from(error: String) -> Self {
        GraphError::Unknown(error)
    }
}

impl<Idx> From<petgraph::algo::Cycle<Idx>> for GraphError
where
    Idx: Copy + std::fmt::Debug,
{
    fn from(error: petgraph::algo::Cycle<Idx>) -> Self {
        GraphError::Cycle(CycleError::Cycle {
            id: format!("{:?}", error.node_id()),
        })
    }
}

impl From<petgraph::algo::NegativeCycle> for GraphError {
    fn from(_error: petgraph::algo::NegativeCycle) -> Self {
        GraphError::Cycle(CycleError::NegativeCylce)
    }
}

#[derive(Clone, Debug, Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case", untagged)
)]
#[repr(usize)]
#[strum(serialize_all = "snake_case")]
pub enum CycleError {
    Cycle { id: String },
    NegativeCylce,
}

macro_rules! into_error {
    ($error:ident, $kind:ident) => {
        impl From<$error> for GraphError {
            fn from(error: $error) -> Self {
                GraphError::$kind(error)
            }
        }
    };
}

into_error!(CycleError, Cycle);
