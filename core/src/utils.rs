/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{Host, Port};
use std::net::SocketAddr;

/// Simple function wrapper for creating [SocketAddr] from its pieces: ([Host], [Port])
pub fn socket_address(host: Host, port: Port) -> SocketAddr {
    SocketAddr::from((host, port))
}
