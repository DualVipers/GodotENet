use crate::{
    Layer, LayerReturn,
    event::{Event, EventType},
    layer_err,
    packet::Packet,
};
use log::debug;

/// A [`Layer`](crate::Layer) which automatically parses incoming packets
/// and adds the parsed packet to the [`DataPile`](crate::DataPile).
///
/// Please note that this [`Layer`](crate::Layer) does not parse rpc packets fully as that requires more data,
/// use the [`RPCParseLayer`](crate::layers::RPCParseLayer) for that functionality.
pub struct AutoParseLayer;

impl Layer for AutoParseLayer {
    fn call(&self, mut event: Event) -> LayerReturn {
        return Box::pin(async move {
            let EventType::Receive { ref raw_packet, .. } = event.event else {
                return Ok(Some(event));
            };

            let parsed_packet: Packet = crate::packet::parse_packet(raw_packet.data())
                .map_err(|e| layer_err!("Error Parsing Packet: {}", e))?;

            if let Ok(message) = str::from_utf8(raw_packet.data()) {
                debug!("Received packet: {:?}", message);
            } else if let Err(error) = str::from_utf8(raw_packet.data()) {
                debug!("Received non-UTF8 packet: {:?}", raw_packet.data());

                debug!("UTF8 Error: {:?}", error);
            }

            debug!("Parsed Header: {:?}", parsed_packet);

            event.data_pile.insert(parsed_packet);

            return Ok(Some(event));
        });
    }
}
