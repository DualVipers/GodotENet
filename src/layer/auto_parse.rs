use crate::{
    GodotENetLayer,
    event::{GodotENetEvent, GodotENetEventType},
    packet::GodotENetPacket,
};
use log::debug;

pub struct AutoParseLayer;

impl GodotENetLayer for AutoParseLayer {
    fn call(
        &self,
        mut event: GodotENetEvent,
    ) -> std::pin::Pin<Box<dyn Future<Output = super::GodotENetLayerResult> + Send + Sync>> {
        return Box::pin(async move {
            if let GodotENetEventType::Receive { ref raw_packet, .. } = event.event {
                let parsed_packet: GodotENetPacket =
                    crate::packet::parse_packet(raw_packet.data()).unwrap();

                if let Ok(message) = str::from_utf8(raw_packet.data()) {
                    debug!("Received packet: {:?}", message);
                    debug!("Parsed Header: {:?}", parsed_packet);
                }

                event.data_pile.insert(parsed_packet);
            }

            return Ok(Some(event));
        });
    }
}
