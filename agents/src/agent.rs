/*
    Appellation: agent <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::specs::{AgentProgram, Architecture};

pub trait Agent {
    type Arch: Architecture;
    type Program: AgentProgram;
}
