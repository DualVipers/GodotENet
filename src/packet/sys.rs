use super::Packet;
use crate::GDPeerID;

const SYS_CMD_SIZE: usize = 6;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SysCommandPacket {
    pub sys_cmd: SysCommand,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// Yoinked from Godot's SceneMultiplayer::_process_packet
pub enum SysCommand {
    SysCommandAuth(SysAuthCommand),
    /// Parsing and Generating Complete
    SysCommandAddPeer(GDPeerID),
    /// Parsing and Generating Complete
    SysCommandDelPeer(GDPeerID),
    SysCommandRelay {
        content: Box<[u8]>,

        gdpeer: GDPeerID,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SysAuthCommand {
    /// Sent by Client to Server to Authenticate
    ///
    /// Respond with CompleteNotification on Success, or Disconnect/Timeout on Failure
    AuthMessage(Box<[u8]>),
    /// Sent by Server to Client to Accept Authentication,
    /// Rejections Should Be Disconnected or Timed Out
    CompleteNotification,
}

// Heavily Uses SceneMultiplayer::_process_sys() in Godot to revese engineer the header
pub fn parse_packet(packet: &[u8]) -> Result<Packet, String> {
    if packet.len() < SYS_CMD_SIZE && packet.len() < 1 && packet[1] != 0 {
        return Err("Packet too short to contain Godot ENet sys command header".to_string());
    }

    let sys_cmd = match packet[1] {
        0 => parse_auth_command(packet)?,
        1 => SysCommand::SysCommandAddPeer(GDPeerID(u32::from_le_bytes([
            packet[2], packet[3], packet[4], packet[5],
        ]) as i32)),
        2 => SysCommand::SysCommandDelPeer(GDPeerID(u32::from_le_bytes([
            packet[2], packet[3], packet[4], packet[5],
        ]) as i32)),
        3 => parse_relay_command(packet)?,
        _ => return Err(format!("Invalid value for Sys command: {}", packet[1])),
    };

    Ok(Packet::NetworkCommandSys(SysCommandPacket { sys_cmd }))
}

fn parse_relay_command(packet: &[u8]) -> Result<SysCommand, String> {
    // Make sure the packet still contains content to relay
    if packet.len() < SYS_CMD_SIZE + 1 {
        return Err("Packet too short to contain Godot ENet sys relay command header".to_string());
    }

    Ok(SysCommand::SysCommandRelay {
        gdpeer: GDPeerID(u32::from_le_bytes([packet[2], packet[3], packet[4], packet[5]]) as i32),

        content: (packet[SYS_CMD_SIZE..]).into(),
    })
}

// Uses SceneMultiplayer::poll() in Godot to reverse engineer the auth command
fn parse_auth_command(packet: &[u8]) -> Result<SysCommand, String> {
    if packet.len() < 2 {
        return Err("Packet too short to contain Godot ENet sys auth command".to_string());
    }

    if packet.len() == 2 {
        Ok(SysCommand::SysCommandAuth(
            SysAuthCommand::CompleteNotification,
        ))
    } else {
        Ok(SysCommand::SysCommandAuth(SysAuthCommand::AuthMessage(
            (packet[2..]).into(),
        )))
    }
}

// Reverse of parse_packet
pub fn gen_packet(packet: &SysCommandPacket) -> Result<Vec<u8>, String> {
    let mut out_packet: Vec<u8> = Vec::new();

    out_packet.push(7); // CMD_MASK for Sys Command

    match packet.sys_cmd {
        SysCommand::SysCommandAuth(_) => {
            out_packet.push(0);
        }
        SysCommand::SysCommandAddPeer(_) => {
            out_packet.push(1);
        }
        SysCommand::SysCommandDelPeer(_) => {
            out_packet.push(2);
        }
        SysCommand::SysCommandRelay { .. } => {
            out_packet.push(3);
        }
    }

    match &packet.sys_cmd {
        SysCommand::SysCommandAuth(SysAuthCommand::AuthMessage(auth_data)) => {
            out_packet.extend(auth_data.as_ref());
        }
        SysCommand::SysCommandAuth(SysAuthCommand::CompleteNotification) => {
            // No additional data
        }
        SysCommand::SysCommandAddPeer(gdpeer) => {
            out_packet.extend(&gdpeer.0.to_le_bytes());
        }
        SysCommand::SysCommandDelPeer(gdpeer) => {
            out_packet.extend(&gdpeer.0.to_le_bytes());
        }
        SysCommand::SysCommandRelay { content, gdpeer } => {
            out_packet.extend(&gdpeer.0.to_le_bytes());
            out_packet.extend(content);
        }
    }

    Ok(out_packet)
}
