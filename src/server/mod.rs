use log::{debug, warn};
use rusty_enet as enet;
use std::net::{SocketAddr, UdpSocket};

pub mod builder;

pub struct GodotENetServer {
    host: Option<enet::Host<UdpSocket>>,

    address: SocketAddr,
}

impl GodotENetServer {
    /// Create a new GodotENetServer builder
    pub fn builder() -> builder::GodotENetServerBuilder {
        builder::GodotENetServerBuilder::default()
    }

    pub fn open(&mut self) -> Result<(), String> {
        self.open_custom(enet::HostSettings {
            peer_limit: 32,
            channel_limit: 2,
            compressor: None,
            checksum: None,
            ..Default::default()
        })?;

        Ok(())
    }

    /// Start the server
    pub fn open_limits(&mut self, peer_limit: usize, channel_limit: usize) -> Result<(), String> {
        self.open_custom(enet::HostSettings {
            peer_limit: peer_limit,
            channel_limit: channel_limit,
            compressor: None,
            checksum: None,
            ..Default::default()
        })?;

        Ok(())
    }

    /// Start the server
    pub fn open_custom(&mut self, enet_settings: enet::HostSettings) -> Result<(), String> {
        // Open Socket and bind to address
        debug!("Binding to address: {}", self.address);
        let socket = UdpSocket::bind(self.address)
            .map_err(|e| format!("Failed to bind to address: {}", e))?;

        if enet_settings.compressor.is_some() {
            warn!(
                "Custom compressor set. Ensure compatibility with Godot's compression algorithm."
            );
        }

        if enet_settings.checksum.is_some() {
            warn!(
                "Custom checksum function set. Ensure compatibility with Godot's checksum algorithm."
            );
        }

        // Create ENet Host
        self.host = Some(enet::Host::new(socket, enet_settings).map_err(|e| {
            format!(
                "Failed to create ENet host with address {}: {}",
                self.address, e
            )
        })?);

        Ok(())
    }

    /// Check if the server is currently open
    pub fn is_open(&self) -> bool {
        self.host.is_some()
    }

    /// Obtain a reference to the ENet host
    pub fn get_host(&mut self) -> Result<&enet::Host<UdpSocket>, String> {
        let host = self.get_mut_host()?;

        Ok(host)
    }

    /// Obtain a mutable reference to the ENet host
    pub fn get_mut_host(&mut self) -> Result<&mut enet::Host<UdpSocket>, String> {
        if self.host.is_none() {
            return Err("Server is not open".to_string());
        }

        Ok(self.host.as_mut().unwrap())
    }
}
