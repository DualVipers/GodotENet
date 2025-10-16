use crate::{
    packet::{Packet, RemoteCacheID},
    utils::clean_path,
};

// Based on Godot's SceneMultiplayer::_process_packet
pub fn parse_packet(packet: &[u8]) -> Result<Packet, String> {
    if packet.len() < 38 {
        return Err("Packet too short to contain Godot ENet packet".to_string());
    }

    let methods_md5_hash = String::from_utf8(packet[1..33].to_vec())
        .map_err(|_| "Failed to parse method_hash as UTF-8".to_string())?;

    let remote_cache_id = u32::from_le_bytes([packet[34], packet[35], packet[36], packet[37]]);

    let path = clean_path(
        String::from_utf8(packet[38..].to_vec())
            .map_err(|_| "Failed to parse path as UTF-8".to_string())?,
    );

    Ok(Packet::NetworkCommandSimplifyPath {
        methods_md5_hash,
        remote_cache_id,
        path,
    })
}

// Reverse of parse_packet
pub fn gen_packet(
    methods_md5_hash: &str,
    remote_cache_id: RemoteCacheID,
    path: &str,
) -> Result<Vec<u8>, String> {
    if methods_md5_hash.len() != 32 {
        return Err("methods_md5_hash must be exactly 32 characters long".to_string());
    }
    let mut out_packet: Vec<u8> = Vec::new();

    out_packet.push(1); // CMD_MASK for Simplify Path

    out_packet.extend(methods_md5_hash.as_bytes());

    out_packet.push(0); // Null Terminator for methods_md5_hash

    out_packet.extend(&remote_cache_id.to_le_bytes());

    out_packet.extend(path.as_bytes());

    out_packet.push(0); // Null Terminator

    Ok(out_packet)
}
