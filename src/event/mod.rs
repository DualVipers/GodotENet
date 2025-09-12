use crate::packet::outgoing::OutgoingPacket;
use rusty_enet as enet;
use std::sync::mpsc;

#[derive(Clone, Debug)]
pub struct Event {
    pub peer_id: enet::PeerID,

    pub event: EventType,

    pub data_pile: super::DataPile,

    pub tx_outgoing: mpsc::Sender<OutgoingPacket>,
}

#[derive(Clone, Debug)]
pub enum EventType {
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
