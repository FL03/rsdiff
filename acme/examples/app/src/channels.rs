/*
    Appellation: channels <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{Context, State};
use scsys::prelude::Locked;
use tokio::sync::{broadcast, mpsc, oneshot};

pub type BroadcastChannels<T> = (broadcast::Sender<T>, broadcast::Receiver<T>);

pub type OneshotChannels<T> = (oneshot::Sender<T>, oneshot::Receiver<T>);

pub type UnboundedMPSC<T> = (mpsc::UnboundedSender<T>, mpsc::UnboundedReceiver<T>);

#[derive(Debug)]
pub struct AppChannels {
    pub ctx: BroadcastChannels<Context>,
    pub state: UnboundedMPSC<Locked<State>>,
}

impl AppChannels {
    pub fn new() -> Self {
        let ctx = broadcast::channel(3);
        let state = mpsc::unbounded_channel();
        Self { ctx, state }
    }
    pub fn context(&self) -> &BroadcastChannels<Context> {
        &self.ctx
    }
    pub fn state_channels(&self) -> &UnboundedMPSC<Locked<State>> {
        &self.state
    }
}

impl Default for AppChannels {
    fn default() -> Self {
        Self::new()
    }
}
