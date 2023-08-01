/*
    Appellation: agents <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::agent::*;

pub(crate) mod agent;

pub trait Agent {
    type Action;
    type Observation;
    type Reward;
    type State;


}

pub trait AgentContext {
    type Params;

}
pub trait Actuator {
    type Action;
    type Env: Environment;

    fn act(&self, action: Self::Action, env: &mut Self::Env) -> &mut Self::Env;
}

pub trait Sensor {

}

pub trait Environment {

}

pub trait AgentFunction {

}
