pub use rusty_enet::Packet;
pub use rusty_enet::PeerID;

#[derive(Clone, Debug)]
/// Packet leaving the server.
pub struct OutgoingPacket {
    pub peer_id: PeerID,
    pub channel_id: u8,
    pub packet: Packet,
}
