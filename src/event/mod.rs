use std::sync::mpsc;

use rusty_enet as enet;

#[derive(Clone, Debug)]
pub struct GodotENetEvent {
    pub peer_id: enet::PeerID,

    pub event: GodotENetEventType,

    pub tx_outgoing: mpsc::Sender<super::OutgoingPacket>,
}

#[derive(Clone, Debug)]
pub enum GodotENetEventType {
    Connect {
        godot_peer: super::GDPeerID,
    },
    Disconnect {
        godot_peer: super::GDPeerID,
    },
    Receive {
        channel_id: u8,
        raw_packet: enet::Packet,
    },
}
