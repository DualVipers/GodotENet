const SYS_CMD_SIZE: usize = 6;

use super::GodotENetPacket;
use crate::GDPeerID;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SysCommandPacket<'a> {
    pub gdpeer: GDPeerID,
    pub sys_cmd: SysCommand<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// Yoinked from Godot's SceneMultiplayer::_process_packet
pub enum SysCommand<'a> {
    SysCommandAuth,
    /// Parsing and Generating Complete
    SysCommandAddPeer,
    /// Parsing and Generating Complete
    SysCommandDelPeer,
    SysCommandRelay {
        content: &'a [u8],
    },
}

// Heavily Uses SceneMultiplayer::_process_sys() in Godot to revese engineer the header
pub fn parse_packet(packet: &[u8]) -> Result<GodotENetPacket, String> {
    if packet.len() < SYS_CMD_SIZE {
        return Err("Packet too short to contain Godot ENet sys command header".to_string());
    }

    let gdpeer: i32 = u32::from_le_bytes([packet[2], packet[3], packet[4], packet[5]]) as i32;

    let sys_cmd = match packet[1] {
        0 => SysCommand::SysCommandAuth, // TODO: Reverse Engineer Auth Shit
        1 => SysCommand::SysCommandAddPeer,
        2 => SysCommand::SysCommandDelPeer,
        3 => parse_relay_command(packet)?,
        _ => return Err(format!("Invalid value for Sys command: {}", packet[1])),
    };

    Ok(GodotENetPacket::NetworkCommandSys(SysCommandPacket {
        gdpeer: GDPeerID(gdpeer),
        sys_cmd,
    }))
}

fn parse_relay_command(packet: &[u8]) -> Result<SysCommand, String> {
    // Make sure the packet still contains content to relay
    if packet.len() < SYS_CMD_SIZE + 1 {
        return Err("Packet too short to contain Godot ENet sys relay command header".to_string());
    }

    Ok(SysCommand::SysCommandRelay {
        content: &packet[SYS_CMD_SIZE..],
    })
}

// Reverse of parse_packet
pub fn gen_packet(packet: &SysCommandPacket) -> Result<Vec<u8>, String> {
    let mut out_packet: Vec<u8> = Vec::new();

    out_packet.push(7); // CMD_MASK for Sys Command

    match packet.sys_cmd {
        SysCommand::SysCommandAuth => {
            out_packet.push(0);
        }
        SysCommand::SysCommandAddPeer => {
            out_packet.push(1);
        }
        SysCommand::SysCommandDelPeer => {
            out_packet.push(2);
        }
        SysCommand::SysCommandRelay { .. } => {
            out_packet.push(3);
        }
    }

    out_packet.extend(&packet.gdpeer.0.to_le_bytes());

    match packet.sys_cmd {
        SysCommand::SysCommandRelay { content } => {
            out_packet.extend(content);
        }
        _ => {} // TODO: Implement other sys commands
    }

    Ok(out_packet)
}
