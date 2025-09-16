pub use rusty_enet::Packet;

use crate::ENetPeerID;

#[derive(Clone, Debug)]
/// Packet leaving the server.
pub struct OutgoingPacket {
    pub peer_id: ENetPeerID,
    pub channel_id: u8,
    pub packet: Packet,
}
