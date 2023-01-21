/*
    Appellation: server <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{ServerOperators, ServerSpec};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Server {
    pub host: [u8; 4],
    pub port: u16,
}

impl Server {
    pub fn new(host: Option<[u8; 4]>, port: Option<u16>) -> Self {
        Self {
            host: host.unwrap_or([0, 0, 0, 0]),
            port: port.unwrap_or(8080),
        }
    }
}

impl ServerOperators for Server {
    fn host(&self) -> [u8; 4] {
        self.host
    }

    fn port(&self) -> u16 {
        self.port
    }
}

impl ServerSpec for Server {
    fn address(&self) -> SocketAddr {
        SocketAddr::from((self.host, self.port))
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new(Some([0, 0, 0, 0]), Some(8080))
    }
}

impl From<([u8; 4], u16)> for Server {
    fn from(data: ([u8; 4], u16)) -> Self {
        Self::new(Some(data.0), Some(data.1))
    }
}

impl From<u16> for Server {
    fn from(data: u16) -> Self {
        Self::new(None, Some(data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server() {
        let server = Server::default();
        assert_eq!(server.host, [0, 0, 0, 0]);
        assert_eq!(server.port, 8080);
    }
}
