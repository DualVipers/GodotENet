pub mod builder;

use crate::{
    DataPile, ENetPeerID, Layer,
    event::{Event, EventType},
    packet::outgoing::OutgoingPacket,
};
use log::{debug, error, info, warn};
use rusty_enet as enet;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, mpsc};
use std::usize;

pub struct Server {
    host: Option<enet::Host<UdpSocket>>,

    address: SocketAddr,

    layers: Arc<Vec<Arc<dyn Layer>>>,

    tx_outgoing: mpsc::Sender<OutgoingPacket>,
    rx_outgoing: mpsc::Receiver<OutgoingPacket>,
}

impl Server {
    /// Create a new GodotENetServer builder
    pub fn builder() -> builder::ServerBuilder {
        builder::ServerBuilder::default()
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

    /// Checks for events on the host and shuttles packets between the host and its peers.
    ///
    /// Returns whether an event was processed.
    ///
    /// Should be called fairly regularly for adequate performance.
    pub async fn service(&mut self) -> Result<bool, String> {
        let host = self.get_mut_host()?;

        if let Some(event) = host.service().unwrap() {
            let enet_peer_id: ENetPeerID;
            let godot_enet_event_data;

            // Build PeerID and GodotENetEventType
            match event {
                enet::Event::Connect { peer, data } => {
                    info!("Peer {:?} connected with {:?}", peer.id().0, data);

                    enet_peer_id = peer.id().into();
                    godot_enet_event_data = EventType::Connect {
                        godot_peer: super::GDPeerID::from(data),
                    };
                }
                enet::Event::Disconnect { peer, data } => {
                    info!("Peer {:?} disconnected with {:?}", peer.id().0, data);

                    enet_peer_id = peer.id().into();
                    godot_enet_event_data = EventType::Disconnect {
                        godot_peer: super::GDPeerID::from(data),
                    };
                }
                enet::Event::Receive {
                    peer,
                    channel_id,
                    packet,
                } => {
                    enet_peer_id = peer.id().into();
                    godot_enet_event_data = EventType::Receive {
                        channel_id,
                        raw_packet: packet,
                    };
                }
            }

            let godot_enet_event = Event {
                peer_id: enet_peer_id,

                event: godot_enet_event_data,

                data_pile: DataPile::default(),

                tx_outgoing: self.tx_outgoing.clone(),
            };

            self.process_event(godot_enet_event).await;
        }

        while let Ok(outgoing) = self.rx_outgoing.try_recv() {
            self.send_outgoing(outgoing).await?;
        }

        return Ok(true);
    }

    /// Process and event through layers by spawning an async task
    async fn process_event(&mut self, mut event: Event) {
        let layers = Arc::clone(&self.layers);
        let mut i: usize = 0;

        tokio::spawn(async move {
            while i < layers.len() {
                let layer = &layers[i];
                i += 1;

                let result = layer.call(event.clone()).await;

                match result {
                    Ok(passed_event) => {
                        if passed_event.is_none() {
                            return;
                        }

                        event = passed_event.unwrap();
                    }
                    Err(e) => {
                        error!("Error processing event in layer: \n{}", e);
                    }
                }
            }
        });
    }

    async fn send_outgoing(&mut self, outgoing: OutgoingPacket) -> Result<(), String> {
        let host = self.get_mut_host()?;

        let peer = host.get_peer_mut(outgoing.peer_id.into()).ok_or_else(|| {
            format!(
                "Failed to find peer with id {:?} to send packet to",
                outgoing.peer_id
            )
        })?;

        if peer.state() != enet::PeerState::Connected {
            return Err(format!(
                "Peer with id {:?} is not connected",
                outgoing.peer_id
            ));
        }

        debug!(
            "Sending packet: {:?}\nto peer: {:?}\non: {:?}",
            outgoing.packet.data(),
            outgoing.peer_id,
            outgoing.channel_id
        );

        peer.send(outgoing.channel_id, &outgoing.packet)
            .map_err(|e| {
                format!(
                    "Failed to send packet to peer {:?}: {:?}",
                    outgoing.peer_id, e
                )
            })?;

        Ok(())
    }

    /// Obtain a reference to the ENet host
    pub fn get_host(&self) -> Result<&enet::Host<UdpSocket>, String> {
        if self.is_open() == false {
            return Err("Server is not open".to_string());
        }

        Ok(self.host.as_ref().unwrap())
    }

    /// Obtain a mutable reference to the ENet host
    pub fn get_mut_host(&mut self) -> Result<&mut enet::Host<UdpSocket>, String> {
        if self.is_open() == false {
            return Err("Server is not open".to_string());
        }

        Ok(self.host.as_mut().unwrap())
    }
}
