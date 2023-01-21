/*
    Appellation: application <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::AsyncSpawnable;
use scsys::prelude::{AsyncResult, Configurable, Contextual, Locked};

/// Implements the base interface for creating compatible platform applications
pub trait AppSpec<Cnf: Configurable>: Default + AsyncSpawnable {
    type Ctx: Contextual;
    type State;
    fn init() -> Self;
    fn context(&self) -> Self::Ctx;
    fn name(&self) -> String;
    fn settings(&self) -> Cnf;
    fn setup(&mut self) -> AsyncResult<&Self>;
    fn slug(&self) -> String {
        self.name().to_ascii_lowercase()
    }
    fn state(&self) -> &Locked<Self::State>;
}

pub struct Application<Ctx: Contextual + Default>(Ctx);

impl<Ctx: Contextual + Default> Application<Ctx> {
    pub fn new() -> Self {
        Self(Default::default())
    }
}
