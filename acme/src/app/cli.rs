/*
    Appellation: cli <specs>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use clap::{Parser, Subcommand};

pub trait CLIAction {
    type Action: clap::FromArgMatches;
}

///
pub trait Commands: Clone + Default + Subcommand {
    fn command(&self) -> Self
    where
        Self: Sized,
    {
        self.clone()
    }
}

///
pub trait CLISpec: Parser {
    type Cmds: Commands;

    fn new() -> Self {
        Self::parse()
    }
    fn command(&self) -> Option<Self::Cmds>
    where
        Self: Sized;
}
