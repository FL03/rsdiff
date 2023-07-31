/*
    Appellation: actor <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Actor
//! 
//! An actor 
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Actor<S> {
    pub state: S
}
