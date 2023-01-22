/*
   Appellation: cmd <cli>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::system::System;
pub(crate) mod system;

use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Subcommand)]
pub enum Commands {
    System(System)
}

impl Default for Commands {
    fn default() -> Self {
        Self::System(Default::default())
    }
}
