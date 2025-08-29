use super::GodotENetPacket;

// Heavily Uses SceneMultiplayer::_process_sys() in Godot to revese engineer the header
pub fn parse_packet(packet: &[u8]) -> Result<GodotENetPacket, String> {
    Ok(GodotENetPacket::NetworkCommandRaw {
        content: &packet[1..],
    })
}

// Reverse of parse_packet
pub fn gen_packet(packet: &[u8]) -> Result<Vec<u8>, String> {
    let mut out_packet: Vec<u8> = Vec::new();

    out_packet.push(3); // CMD_MASK for Raw Command
    out_packet.extend_from_slice(packet);

    Ok(out_packet)
}
