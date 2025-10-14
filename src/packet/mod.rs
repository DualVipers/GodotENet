pub mod confirm_path;
pub mod outgoing;
pub mod raw;
pub mod rpc;
pub mod simplify_path;
pub mod sys;

use crate::packet::rpc::RPCCommand;

// From scene_multiplayer.h
const CMD_MASK: u8 = 0x7;

pub type RemoteCacheID = u32;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// Yoinked from Godot's SceneMultiplayer::_process_packet
pub enum Packet {
    NetworkCommandRemoteCall(rpc::RPCCommandHeader),
    NetworkCommandSimplifyPath {
        methods_md5_hash: String, // 32 bytes
        remote_cache_id: RemoteCacheID,
        path: String, // Variable Length
    },
    NetworkCommandConfirmPath {
        valid_rpc_checksum: bool,
        remote_cache_id: RemoteCacheID,
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
pub fn parse_packet(packet: &[u8]) -> Result<Packet, String> {
    if packet.len() < 1 {
        return Err("Packet too short to contain Godot ENet packet".to_string());
    }

    // TODO: Remaining packets
    match packet[0] & CMD_MASK {
        0 => rpc::parse_packet(packet),
        1 => simplify_path::parse_packet(packet),
        2 => confirm_path::parse_packet(packet),
        3 => raw::parse_packet(packet),
        4 => Ok(Packet::NetworkCommandSpawn),
        5 => Ok(Packet::NetworkCommandDespawn),
        6 => Ok(Packet::NetworkCommandSync),
        7 => sys::parse_packet(packet),
        _ => Err(format!(
            "Invalid value for Packet command: {}",
            packet[0] & CMD_MASK
        )),
    }
}

// Reverse of parse_packet
/// Generate a Godot ENet Packet from the provided data
pub fn gen_packet(packet: &Packet) -> Result<Vec<u8>, String> {
    match packet {
        Packet::NetworkCommandRemoteCall(header) => rpc::gen_packet(
            header,
            &RPCCommand {
                path: String::new(),
                args: Vec::new(),
            },
        ),
        Packet::NetworkCommandSimplifyPath {
            methods_md5_hash, // 32 bytes
            remote_cache_id,
            path,
        } => simplify_path::gen_packet(methods_md5_hash, *remote_cache_id, path),
        Packet::NetworkCommandConfirmPath {
            valid_rpc_checksum,
            remote_cache_id,
        } => confirm_path::gen_packet(*valid_rpc_checksum, *remote_cache_id),
        Packet::NetworkCommandRaw { content } => raw::gen_packet(content),
        Packet::NetworkCommandSpawn => todo!(), // TODO: Remaining packets
        Packet::NetworkCommandDespawn => todo!(),
        Packet::NetworkCommandSync => todo!(),
        Packet::NetworkCommandSys(packet) => sys::gen_packet(packet),
    }
}

// TODO: Custom Packet Parse Error?
