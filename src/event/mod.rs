use rusty_enet as enet;
use std::sync::mpsc;

#[derive(Clone, Debug)]
pub struct GodotENetEvent {
    pub peer_id: enet::PeerID,

    pub event: GodotENetEventType,

    pub data_pile: super::DataPile,

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
