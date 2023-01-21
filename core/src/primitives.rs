/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{constants::*, types::*};

pub(crate) mod constants {
    ///
    pub const ACCEPT_APP_JSON: &str = "application/json";
    ///
    pub const ME_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36 Edg/91.0.864.59";
    ///
    pub const LOCALHOST: super::Host = [127, 0, 0, 1];
}

pub(crate) mod types {
    ///
    pub type ChannelPackStd<T> = (std::sync::mpsc::Sender<T>, std::sync::mpsc::Receiver<T>);
    ///
    pub type Host = [u8; 4];
    ///
    pub type Port = u16;
    ///
    pub type ServerAddr = std::net::SocketAddr;
    ///
    pub type SocketAddrPieces = (Host, Port);
    ///
    pub type TokioChannelPackMPSC<T> =
        (tokio::sync::mpsc::Sender<T>, tokio::sync::mpsc::Receiver<T>);
}
