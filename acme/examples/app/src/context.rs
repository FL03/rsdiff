/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{OneshotChannels, Settings, UnboundedMPSC};
use decanter::prelude::{Hash, Hashable};
use scsys::prelude::{Contextual, SerdeDisplay};
use serde::{Deserialize, Serialize};
use std::{convert::From, path::PathBuf};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, SerdeDisplay, Serialize)]
pub struct Context {
    pub cnf: Settings,
    pub workdir: PathBuf,
}

impl Context {
    pub fn new(cnf: Option<Settings>, workdir: Option<PathBuf>) -> Self {
        Self {
            cnf: cnf.unwrap_or_default(),
            workdir: workdir.unwrap_or_else(scsys::project_root),
        }
    }
    pub fn settings(&self) -> &Settings {
        &self.cnf
    }
    pub fn set_settings(&mut self, cnf: Settings) -> &Self {
        self.cnf = cnf;
        self
    }
    pub fn workdir(&self) -> &PathBuf {
        &self.workdir
    }
}

impl Contextual for Context {
    type Cnf = Settings;

    type Ctx = Self;

    fn context(&self) -> &Self::Ctx {
        self
    }
}

impl From<Context> for OneshotChannels<Context> {
    fn from(_val: Context) -> Self {
        tokio::sync::oneshot::channel()
    }
}

impl From<Context> for UnboundedMPSC<Context> {
    fn from(_val: Context) -> Self {
        tokio::sync::mpsc::unbounded_channel()
    }
}

impl From<Settings> for Context {
    fn from(data: Settings) -> Self {
        Self::new(Some(data), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let a = Context::default();
        let b = a.clone();
        assert_eq!(a, b)
    }
}
