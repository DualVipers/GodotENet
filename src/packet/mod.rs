pub mod raw;
pub mod rpc;
pub mod sys;

// From scene_multiplayer.h
const CMD_MASK: u8 = 0x7;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
// Yoinked from Godot's SceneMultiplayer::_process_packet
pub enum GodotENetPacket<'a> {
    NetworkCommandRemoteCall(rpc::RPCCommandHeader),
    NetworkCommandSimplifyPath,
    NetworkCommandConfirmPath,
    NetworkCommandRaw { content: &'a [u8] },
    NetworkCommandSpawn,
    NetworkCommandDespawn,
    NetworkCommandSync,
    NetworkCommandSys(sys::SysCommandPacket<'a>),
}

// Baseed on Godot's SceneMultiplayer::_process_packet
/// Parses the provided Godot ENet Packet data
pub fn parse_packet(packet: &[u8]) -> Result<GodotENetPacket, String> {
    if packet.len() < 1 {
        return Err("Packet too short to contain Godot ENet packet".to_string());
    }

    // TODO: Remaining packets
    match packet[0] & CMD_MASK {
        0 => rpc::parse_packet(packet),
        1 => Ok(GodotENetPacket::NetworkCommandSimplifyPath),
        2 => Ok(GodotENetPacket::NetworkCommandConfirmPath),
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
        GodotENetPacket::NetworkCommandSimplifyPath => todo!(),
        GodotENetPacket::NetworkCommandConfirmPath => todo!(),
        GodotENetPacket::NetworkCommandRaw { content } => raw::gen_packet(content),
        GodotENetPacket::NetworkCommandSpawn => todo!(),
        GodotENetPacket::NetworkCommandDespawn => todo!(),
        GodotENetPacket::NetworkCommandSync => todo!(),
        GodotENetPacket::NetworkCommandSys(packet) => sys::gen_packet(packet),
    }
}
