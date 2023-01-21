/*
    Appellation: session <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::events::Event;
use scsys::prelude::Timestamp;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Session {
    pub events: Vec<Event>,
    pub timestamp: i64,
}

impl Session {
    pub fn new(events: Vec<Event>, timestamp: i64) -> Self {
        Self { events, timestamp }
    }
}

impl From<&Session> for Session {
    fn from(data: &Session) -> Self {
        data.clone()
    }
}

impl From<Vec<Event>> for Session {
    fn from(data: Vec<Event>) -> Self {
        Self::new(data, Timestamp::default().into())
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new(Vec::new(), Timestamp::default().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_session() {
        let a = Session::default();
        let b = Session::from(&a);
        assert_eq!(&a, &b)
    }
}
