use crate::{
    Layer, LayerResult,
    event::{Event, EventType},
    packet::Packet,
};
use log::debug;

pub struct AutoParseLayer;

impl Layer for AutoParseLayer {
    fn call(
        &self,
        mut event: Event,
    ) -> std::pin::Pin<Box<dyn Future<Output = LayerResult> + Send + Sync>> {
        return Box::pin(async move {
            if let EventType::Receive { ref raw_packet, .. } = event.event {
                let parsed_packet: Packet = crate::packet::parse_packet(raw_packet.data()).unwrap();

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
