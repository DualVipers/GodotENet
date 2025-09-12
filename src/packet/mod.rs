pub mod confirm_path;
pub mod raw;
pub mod rpc;
pub mod simplify_path;
pub mod sys;

// From scene_multiplayer.h
const CMD_MASK: u8 = 0x7;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// Yoinked from Godot's SceneMultiplayer::_process_packet
pub enum GodotENetPacket {
    NetworkCommandRemoteCall(rpc::RPCCommandHeader),
    NetworkCommandSimplifyPath {
        methods_md5_hash: String, // 32 bytes
        remote_cache_id: u32,
        path: String, // Variable Length
    },
    NetworkCommandConfirmPath {
        valid_rpc_checksum: bool,
        remote_cache_id: u32,
    },
    NetworkCommandRaw {
        content: Box<[u8]>,
    },
    NetworkCommandSpawn,
    NetworkCommandDespawn,
    NetworkCommandSync,
    NetworkCommandSys(sys::SysCommandPacket),
}

// Based on Godot's SceneMultiplayer::_process_packet
/// Parses the provided Godot ENet packet data
pub fn parse_packet(packet: &[u8]) -> Result<GodotENetPacket, String> {
    if packet.len() < 1 {
        return Err("Packet too short to contain Godot ENet packet".to_string());
    }

    // TODO: Remaining packets
    match packet[0] & CMD_MASK {
        0 => rpc::parse_packet(packet),
        1 => simplify_path::parse_packet(packet),
        2 => confirm_path::parse_packet(packet),
        3 => raw::parse_packet(packet),
        4 => Ok(GodotENetPacket::NetworkCommandSpawn),
        5 => Ok(GodotENetPacket::NetworkCommandDespawn),
        6 => Ok(GodotENetPacket::NetworkCommandSync),
        7 => sys::parse_packet(packet),
        _ => Err(format!(
            "Invalid value for Packet command: {}",
            packet[0] & CMD_MASK
        )),
    }
}

// Reverse of parse_packet
/// Generate a Godot ENet Packet from the provided data
pub fn gen_packet(packet: &GodotENetPacket) -> Result<Vec<u8>, String> {
    match packet {
        GodotENetPacket::NetworkCommandRemoteCall(header) => rpc::gen_packet(header),
        GodotENetPacket::NetworkCommandSimplifyPath {
            methods_md5_hash, // 32 bytes
            remote_cache_id,
            path,
        } => simplify_path::gen_packet(methods_md5_hash, *remote_cache_id, path),
        GodotENetPacket::NetworkCommandConfirmPath {
            valid_rpc_checksum,
            remote_cache_id,
        } => confirm_path::gen_packet(*valid_rpc_checksum, *remote_cache_id),
        GodotENetPacket::NetworkCommandRaw { content } => raw::gen_packet(content),
        GodotENetPacket::NetworkCommandSpawn => todo!(), // TODO: Remaining packets
        GodotENetPacket::NetworkCommandDespawn => todo!(),
        GodotENetPacket::NetworkCommandSync => todo!(),
        GodotENetPacket::NetworkCommandSys(packet) => sys::gen_packet(packet),
    }
}
