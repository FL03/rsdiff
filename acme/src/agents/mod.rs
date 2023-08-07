/*
    Appellation: agents <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{agent::*, environment::*};

pub(crate) mod agent;
pub(crate) mod environment;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReflexAgent {
    ModelBased = 0,
    #[default]
    Simple = 1,
}

pub enum AgentType {
    Reflex(ReflexAgent),
    GoalBased,
    Utilitarian,
    Learning,
}

pub trait Architecture {
    type Actuator: Actuator;
    type Observer: Observer;
}

pub trait Agent {
    type Arch: Architecture;
    type Program: AgentProgram;
}

pub trait Observer {
    type Observation;
    type Reward;
    type State;

    fn observe(&self) -> (Self::Observation, Self::Reward, Self::State);
}

pub trait AgentContext {
    type Params;
    type State;
    type Store: Send + Sync;

    fn params(&self) -> Self::Params;

    fn state(&self) -> Self::State;

    fn store(&self) -> &Self::Store;
}

pub trait Actuator {
    type Action;
    type Env: Environment;

    fn act(&self, action: Self::Action, env: &mut Self::Env) -> &mut Self::Env;
}

pub trait AgentProgram: AgentFunction {}

pub trait AgentFunction {
    type Action;
    type Env: Environment;
    type Params;

    fn compute(&self, params: Self::Params, env: &mut Self::Env) -> Self::Action;
}
