/*
   Appellation: states <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::compiler::*;

pub(crate) mod compiler;

pub type CompilerState = scsys::prelude::State<CompilerStates>;
