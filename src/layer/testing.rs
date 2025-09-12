// TODO: REMOVE FILE

use crate::{GodotENetLayer, GodotENetLayerReturn, OutgoingPacket, event::GodotENetEvent};
use rusty_enet as enet;

pub struct TestingLayer;

impl GodotENetLayer for TestingLayer {
    fn call(&self, event: GodotENetEvent) -> GodotENetLayerReturn {
        return Box::pin(async move {
            if let Some(parsed_packet) = event.data_pile.get::<crate::packet::GodotENetPacket>() {
                if let crate::packet::GodotENetPacket::NetworkCommandSys(packet) = parsed_packet {
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

                    if let crate::packet::sys::SysCommand::SysCommandRelay { content } =
                        &packet.sys_cmd
                    {
                        let outgoing_packet = OutgoingPacket {
                            peer_id: enet::PeerID(0),
                            channel_id: 0,
                            packet: enet::Packet::reliable((*content).clone()),
                        };

                        if let Err(e) = event.tx_outgoing.send(outgoing_packet) {
                            return Err(format!("Failed to transmit outgoing packet: {:?}", e));
                        }

                        return Ok(Some(event));
                    }

                    return Ok(None);
                }
            }

            return Ok(Some(event));
        });
    }
}
