use crate::{GodotENetLayer, GodotENetServer};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::{Arc, mpsc},
};

pub struct GodotENetServerBuilder {
    address: SocketAddr,

    layers: Vec<Arc<dyn GodotENetLayer + Send + Sync + 'static>>,
}

impl GodotENetServerBuilder {
    pub fn new() -> GodotENetServerBuilder {
        GodotENetServerBuilder {
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 55556),

            layers: Vec::new(),
        }
    }

    pub fn build(self) -> Result<GodotENetServer, String> {
        let (tx_outgoing, rx_outgoing) = mpsc::channel();

        let mut layers: Vec<Arc<dyn GodotENetLayer + Send + Sync>> = Vec::new();

        for layer in self.layers.iter() {
            layers.push(layer.clone());
        }

        let server = GodotENetServer {
            host: None,

            address: self.address,

            layers: Arc::new(layers),

            tx_outgoing: tx_outgoing,
            rx_outgoing: rx_outgoing,
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

// Layer Implementations
impl GodotENetServerBuilder {
    /// Add a layer to the server
    /// Layers are processed in the order they are added
    pub fn layer<T: GodotENetLayer + Sync + Send + 'static>(
        mut self,
        layer: T,
    ) -> GodotENetServerBuilder {
        self.layers.push(Arc::new(layer));

        self
    }

    // TODO: Add Multiple Layers At Once
}

impl Default for GodotENetServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
