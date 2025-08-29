use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use super::GodotENetServer;

pub struct GodotENetServerBuilder {
    address: SocketAddr,
}

impl GodotENetServerBuilder {
    pub fn new() -> GodotENetServerBuilder {
        GodotENetServerBuilder {
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 55556),
        }
    }

    pub fn build(self) -> Result<GodotENetServer, String> {
        let server = GodotENetServer {
            host: None,

            address: self.address,
        };

        Ok(server)
    }
}

// Address Implementations
impl GodotENetServerBuilder {
    /// Set the socket address for the server to bind to
    /// Overrides address and port if previously set
    pub fn socket(mut self, address: SocketAddr) -> GodotENetServerBuilder {
        self.address = address;
        self
    }

    /// Set the address and port for the server to bind to
    pub fn address(mut self, address: &str, port: &str) -> Result<GodotENetServerBuilder, String> {
        self.address = format!("{}:{}", address, port)
            .parse::<SocketAddr>()
            .map_err(|e| format!("Failed to parse socket address: {}", e))?;

        Ok(self)
    }
}

impl Default for GodotENetServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
