/*
    Appellation: address <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Address {
    IpAddr(IpAddr),
    Socket(SocketAddr)
}

impl Address {
    pub fn ipaddr(addr: IpAddr) -> Self {
        Self::from(addr)
    }
    pub fn socket(addr: SocketAddr) -> Self {
        Self::from(addr)
    }
}

impl Default for Address {
    fn default() -> Self {
        Self::from(SocketAddr::from(([0, 0, 0, 0], 8080)))
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.clone() {
            Self::IpAddr(addr) => write!(f, "{}", addr),
            Self::Socket(addr) => write!(f, "{}", addr)
        }
    }
}

impl From<IpAddr> for Address {
    fn from(addr: IpAddr) -> Self {
        Self::IpAddr(addr)
    }
}

impl From<SocketAddr> for Address {
    fn from(addr: SocketAddr) -> Self {
        Self::Socket(addr)
    }
}

impl From<[u8; 4]> for Address {
    fn from(addr: [u8; 4]) -> Self {
        Self::from(IpAddr::from(addr))
    }
}

impl From<([u8; 4], u16)> for Address {
    fn from(addr: ([u8; 4], u16)) -> Self {
        Self::from(SocketAddr::from(addr))
    }
}
