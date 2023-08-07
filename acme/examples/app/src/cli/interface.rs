/*
   Appellation: interface <cli>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::cli::cmd::Commands;
use clap::{ArgAction, Parser};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Parser, PartialEq, Serialize)]
#[clap(about, author, long_about = None, version)]
#[command(arg_required_else_help(true), allow_missing_positional(true))]
pub struct CommandLineInterface {
    #[clap(subcommand)]
    pub command: Option<Commands>,
    #[arg(action = ArgAction::SetTrue, default_value_t = false, long, short)]
    pub debug: bool,
}

impl CommandLineInterface {
    pub fn command(&self) -> Option<Commands> {
        self.command.clone()
    }
    pub fn debug(&self) -> bool {
        self.debug
    }
}

impl Default for CommandLineInterface {
    fn default() -> Self {
        Self::parse()
    }
}
