use super::GodotENetPacket;

// Based on Godot's SceneMultiplayer::_process_packet and SceneCacheInterface::process_confirm_path
pub fn parse_packet(packet: &[u8]) -> Result<GodotENetPacket, String> {
    if packet.len() != 6 {
        return Err("Packet not sized to contain Godot ENet packet".to_string());
    }

    let valid_rpc_checksum = packet[1];

    let remote_cache_id = u32::from_le_bytes([packet[2], packet[3], packet[4], packet[5]]);

    Ok(GodotENetPacket::NetworkCommandConfirmPath {
        valid_rpc_checksum: valid_rpc_checksum != 0,
        remote_cache_id,
    })
}

// Reverse of parse_packet
pub fn gen_packet(valid_rpc_checksum: bool, remote_cache_id: u32) -> Result<Vec<u8>, String> {
    let mut out_packet: Vec<u8> = Vec::new();

    out_packet.push(2); // CMD_MASK for Confirm Path

    out_packet.push(if valid_rpc_checksum { 1 } else { 0 });

    out_packet.extend(&remote_cache_id.to_le_bytes());

    Ok(out_packet)
}
