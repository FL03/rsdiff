/*
   Appellation: system <cli>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use clap::{Args, ArgAction};
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};


#[derive(Args, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct System {
    #[clap(long, short)]
    pub config: Option<std::path::PathBuf>,
    #[arg(action = ArgAction::SetTrue, long, short = 'U')]
    pub up: bool
}


impl System {
    pub async fn handle(&self) -> AsyncResult<&Self> {
        
        Ok(self)
    }
}
