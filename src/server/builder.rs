use crate::{Layer, Server};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::{Arc, mpsc},
};

pub struct ServerBuilder {
    address: SocketAddr,

    layers: Vec<Arc<dyn Layer>>,
}

impl ServerBuilder {
    pub fn new() -> ServerBuilder {
        ServerBuilder {
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 55556),

            layers: Vec::new(),
        }
    }

    pub fn build(self) -> Result<Server, String> {
        let (tx_outgoing, rx_outgoing) = mpsc::channel();

        let mut layers: Vec<Arc<dyn Layer>> = Vec::new();

        for layer in self.layers.iter() {
            layers.push(layer.clone());
        }

        let server = Server {
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
impl ServerBuilder {
    /// Set the socket address for the server to bind to
    /// Overrides address and port if previously set
    pub fn socket(mut self, address: SocketAddr) -> ServerBuilder {
        self.address = address;
        self
    }

    /// Set the address and port for the server to bind to
    pub fn address(mut self, address: &str, port: &str) -> Result<ServerBuilder, String> {
        self.address = format!("{}:{}", address, port)
            .parse::<SocketAddr>()
            .map_err(|e| format!("Failed to parse socket address: {}", e))?;

        Ok(self)
    }
}

// Layer Implementations
impl ServerBuilder {
    /// Add a layer to the server
    ///
    /// Layers are processed in the order they are added
    pub fn layer<T: Layer + Sync + Send + 'static>(mut self, layer: T) -> ServerBuilder {
        self.layers.push(Arc::new(layer));

        self
    }

    /// Add an arc layer to the server
    ///
    /// Layers are processed in the order they are added
    pub fn arc_layer<T: Layer + Sync + Send + 'static>(mut self, layer: Arc<T>) -> ServerBuilder {
        self.layers.push(layer);

        self
    }

    /// Add multiple layers to the server
    ///
    /// Layers are processed in the order they are added
    pub fn layers<T: Layer + Sync + Send + 'static>(mut self, layers: Vec<T>) -> ServerBuilder {
        for layer in layers.into_iter() {
            self.layers.push(Arc::new(layer));
        }

        self
    }

    /// Add multiple arc layers to the server
    ///
    /// Layers are processed in the order they are added
    pub fn arc_layers<T: Layer + Sync + Send + 'static>(
        mut self,
        layers: Vec<Arc<T>>,
    ) -> ServerBuilder {
        for layer in layers.iter() {
            self.layers.push(layer.clone());
        }

        self
    }

    // TODO: Add Multiple Layers At Once, Probably With Some "LayerSet"
}

impl Default for ServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
