// From scene_rpc_interface.h
const NODE_ID_COMPRESSION_SHIFT: u8 = 4;
const NAME_ID_COMPRESSION_SHIFT: u8 = 6;

// From scene_rpc_interface.h
const NODE_ID_COMPRESSION_FLAG: u8 =
    (1 << NODE_ID_COMPRESSION_SHIFT) | (1 << (NODE_ID_COMPRESSION_SHIFT + 1));
const NAME_ID_COMPRESSION_FLAG: u8 = 1 << NAME_ID_COMPRESSION_SHIFT;

use super::GodotENetPacket;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RPCCommandHeader {
    pub node_id_compression: u8,
    pub node_id: u32,
    pub name_id_compression: u8,
    pub name_id: u32,
}

// Heavily Uses SceneRPCInterface::process_rpc() and SceneMultiplayer in Godot to revese engineer the header
pub fn parse_packet(packet: &[u8]) -> Result<GodotENetPacket, String> {
    if packet.len() < 1 {
        return Err("Packet too short to contain Godot ENet header".to_string());
    }

    let node_id_compression = (packet[0] & NODE_ID_COMPRESSION_FLAG) >> NODE_ID_COMPRESSION_SHIFT;
    let name_id_compression = (packet[0] & NAME_ID_COMPRESSION_FLAG) >> NAME_ID_COMPRESSION_SHIFT;

    if packet.len() < 1 + (2 ^ node_id_compression as usize) + (2 ^ name_id_compression as usize) {
        return Err("Packet too short to contain Godot ENet header".to_string());
    }

    let node_id: u32 = match node_id_compression {
        0 => packet[1] as u32,
        1 => u16::from_le_bytes([packet[1], packet[2]]) as u32,
        2 => u32::from_le_bytes([packet[1], packet[2], packet[3], packet[4]]),
        _ => return Err("Invalid node_id_compression value".to_string()),
    };

    // TODO: Implement name_id_compression parsing if needed
    // Node *node = _process_get_node(p_from, p_packet, node_target, p_packet_len);
    // ERR_FAIL_NULL_MSG(node, "Invalid packet received. Requested node was not found.");

    let offset: usize = packet[1 + (1 << node_id_compression) as usize] as usize;
    let name_id: u32 = match name_id_compression {
        0 => packet[offset] as u32,
        1 => u16::from_le_bytes([packet[offset], packet[1 + offset]]) as u32,
        _ => return Err("Invalid node_id_compression value".to_string()),
    };

    // TODO: Get Content of the Packet

    Ok(GodotENetPacket::NetworkCommandRemoteCall(
        RPCCommandHeader {
            node_id_compression,
            node_id,
            name_id_compression,
            name_id,
        },
    ))
}

// Reverse of parse_packet
pub fn gen_packet(packet: &RPCCommandHeader) -> Result<Vec<u8>, String> {
    let mut out_packet: Vec<u8> = Vec::new();

    let mut header_byte: u8 = 0;
    header_byte |=
        (packet.node_id_compression << NODE_ID_COMPRESSION_SHIFT) & NODE_ID_COMPRESSION_FLAG;
    header_byte |=
        (packet.name_id_compression << NAME_ID_COMPRESSION_SHIFT) & NAME_ID_COMPRESSION_FLAG;

    out_packet.push(header_byte);

    match packet.node_id_compression {
        0 => {
            out_packet.push(packet.node_id as u8);
        }
        1 => {
            out_packet.extend_from_slice(&(packet.node_id as u16).to_le_bytes());
        }
        2 => {
            out_packet.extend_from_slice(&packet.node_id.to_le_bytes());
        }
        _ => panic!("Invalid node_id_compression value"),
    }

    match packet.name_id_compression {
        0 => {
            out_packet.push(packet.name_id as u8);
        }
        1 => {
            out_packet.extend_from_slice(&(packet.name_id as u16).to_le_bytes());
        }
        _ => panic!("Invalid name_id_compression value"),
    }

    // TODO: Generate Content of the Packet (Implement Parse First)

    Ok(out_packet)
}
