// TODO: ADD RPC Parsing

use crate::{
    GDPeerID, Layer, LayerReturn,
    event::{Event, EventType},
    layers::PathCache,
    packet::{Packet, rpc::RPCCommand},
    utils::clean_path,
    variant::{self, Variant},
};
use std::sync::Arc;

/// A [`Layer`](crate::Layer) which automatically parses incoming rpc packets
/// and adds the parsed rpc packet to the [`DataPile`](crate::DataPile).
///
/// Depends on [`AutoParseLayer`](crate::layers::AutoParseLayer),
/// [`PeerMapLayer`](crate::layers::PeerMapLayer),
/// and [`PathCacheLayer`](crate::layers::PathCacheLayer).
pub struct RPCParseLayer;

impl Layer for RPCParseLayer {
    fn call(&self, mut event: Event) -> LayerReturn {
        return Box::pin(async move {
            let EventType::Receive { ref raw_packet, .. } = event.event else {
                return Ok(Some(event));
            };

            let parsed_packet = match event.data_pile.get::<Packet>() {
                Some(packet) => packet,
                None => {
                    return Err(
                        "RPCParseLayer: PathCacheLayer ran without parsed packet, requires AutoParseLayer"
                            .to_string(),
                    );
                }
            };

            let Packet::NetworkCommandRemoteCall(header) = parsed_packet else {
                return Ok(Some(event));
            };

            // This section replicates SceneRPCInterface::_process_get_node

            let path;

            let offset = (header.node_id & 0x7FFFFFFF) as usize;

            if (header.node_id & 0x80000000) != 0 {
                // Full Path Sent
                if (offset) > raw_packet.data().len() {
                    return Err(
                        "RPCParseLayer: Full Path RPC Packet too short to contain full path"
                            .to_string(),
                    );
                }

                path = clean_path(
                    String::from_utf8(raw_packet.data()[offset..].to_vec())
                        .map_err(|e| format!("Invalid UTF-8 in Full Path of RPC Packet: {}", e))?,
                );
            } else {
                let peer_id = match event.data_pile.get::<GDPeerID>() {
                    Some(peer_id) => peer_id,
                    None => {
                        return Err("RPCParseLayer: Ran without Godot Peer ID in DataPile, requires PeerMapLayer".to_string());
                    }
                };

                let path_cache = match event.data_pile.get::<PathCache>() {
                    Some(peer_id) => peer_id,
                    None => {
                        return Err("RPCParseLayer: Ran without Path Cache in DataPile, requires PathCacheLayer".to_string());
                    }
                };

                path = path_cache.get_path(peer_id, &header.node_id).ok_or_else(|| {
                        format!(
                            "RPCParseLayer: Could not find path in cache for Godot Peer ID: {:?} with Node ID: {}",
                            peer_id, header.node_id
                        )
                    })?.clone();
            }

            // From Here On This Is Based on SceneRPCInterface::_process_rpc

            let packet_header_offset = 1
                + (1 << header.node_id_compression) as usize
                + (1 << header.name_id_compression) as usize;
            if raw_packet.data().len() < packet_header_offset {
                return Err(
                    "RPCLayerParser: Packet Too Short to Contain Godot ENet RPC Packet".to_string(),
                );
            }

            let mut args: Vec<Arc<Box<dyn Variant>>> = Vec::new();

            if header.byte_only_or_no_args {
                if raw_packet.data().len() > packet_header_offset {
                    let pba = variant::PackedByteArray(
                        raw_packet.data()[packet_header_offset..].to_vec(),
                    );

                    args.push(Arc::new(Box::new(pba)));
                }
            } else {
                // Normal variant, takes the argument count from the packet.
                let argc = raw_packet.data()[packet_header_offset];
                let mut offset = packet_header_offset + 1;

                let mut i = 0;
                while i < argc {
                    if offset >= raw_packet.data().len() {
                        return Err(format!(
                            "RPCParseLayer: Packet Too Short to Contain Argument {} of {}",
                            i + 1,
                            argc
                        ));
                    }

                    let decoding_result = match variant::decode_and_decompress_variant(
                        &raw_packet.data()[offset..],
                    ) {
                        Ok(result) => result,
                        Err(e) => {
                            return Err(format!(
                                "RPCParseLayer: Failed to decode and decompress variant {} of {} in RPC Packet: \n{}",
                                i + 1,
                                argc,
                                e
                            ));
                        }
                    };

                    args.push(decoding_result.variant.into());
                    offset += decoding_result.consumed;

                    i += 1;
                }
            }

            event.data_pile.insert(RPCCommand { path, args });

            return Ok(Some(event));
        });
    }
}
