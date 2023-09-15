/*
    Appellation: agent <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Architecture, AgentProgram};

pub trait Agent {
    type Arch: Architecture;
    type Program: AgentProgram;
}


pub struct SimpleAgent {
    
}