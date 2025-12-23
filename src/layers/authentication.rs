use crate::{
    DataPile, ENetPeerID, Layer, LayerReturn,
    event::{Event, EventType},
    layer_err,
    packet::{
        Packet, gen_packet, outgoing,
        sys::{SysAuthCommand, SysCommand, SysCommandPacket},
    },
};
use dashmap::DashMap;
use std::sync::{Arc, mpsc::Sender};

/// A [`Layer`](crate::Layer) which automatically processes authentication.
///
/// Depends on [`AutoParseLayer`](crate::layers::AutoParseLayer).
///
/// Should run before any layers which require authentication,
/// which should be all others in most cases.
pub struct AuthenticationLayer<F>
where
    F: Future<Output = bool> + Sync + Send + 'static,
{
    pub authentication_callback: fn(ENetPeerID, Box<[u8]>, DataPile) -> F,

    cache: Arc<DashMap<ENetPeerID, bool>>,

    /// Whether to protect unauthed peers by blocking their packets automatically.
    pub protected: bool,

    /// Whether to automatically send blank (\[0x00\]) authentication packets.
    ///
    /// If false, your authentication_callback must send an authentication packet
    /// with some content if returning true.
    pub auto_send_auth: bool,
}

impl<F> AuthenticationLayer<F>
where
    F: Future<Output = bool> + Sync + Send + 'static,
{
    pub fn build(authentication_callback: fn(ENetPeerID, Box<[u8]>, DataPile) -> F) -> Self {
        Self {
            authentication_callback,

            cache: Arc::new(DashMap::new()),

            protected: true,
            auto_send_auth: true,
        }
    }
}

impl<F> Layer for AuthenticationLayer<F>
where
    F: Future<Output = bool> + Sync + Send + 'static,
{
    fn call(&self, mut event: Event) -> LayerReturn {
        let cache = self.cache.clone();
        let protected = self.protected.clone();
        let auto_send_auth = self.auto_send_auth.clone();
        let authentication_callback = self.authentication_callback;

        return Box::pin(async move {
            if let EventType::Disconnect { .. } = event.event {
                // Clean up cache on disconnect
                if let Some(_) = cache.remove(&event.peer_id) {
                    log::info!(
                        "Peer {:?} disconnected, removed from authentication cache",
                        event.peer_id
                    );
                }

                event.data_pile.insert(cache);

                return Ok(Some(event));
            };

            let EventType::Receive { .. } = event.event else {
                event.data_pile.insert(cache);

                return Ok(Some(event));
            };

            let Some(parsed_packet) = event.data_pile.get::<Packet>() else {
                return Err(layer_err!(
                    "Ran without parsed packet, requires AutoParseLayer".to_string()
                ));
            };

            let Packet::NetworkCommandSys(sys_packet) = parsed_packet else {
                event.data_pile.insert(cache.clone());

                return if protected && cache.get(&event.peer_id).is_some_and(|v| !*v) {
                    Ok(None)
                } else {
                    Ok(Some(event))
                };
            };

            let SysCommand::SysCommandAuth(auth_cmd) = &sys_packet.sys_cmd else {
                event.data_pile.insert(cache.clone());

                return if protected && cache.get(&event.peer_id).is_some_and(|v| !*v) {
                    Ok(None)
                } else {
                    Ok(Some(event))
                };
            };

            let SysAuthCommand::AuthMessage(auth_data) = auth_cmd else {
                event.data_pile.insert(cache.clone());

                return if protected && cache.get(&event.peer_id).is_some_and(|v| !*v) {
                    Ok(None)
                } else {
                    Ok(Some(event))
                };
            };

            let authenticated = authentication_callback(
                event.peer_id.clone(),
                auth_data.clone(),
                event.data_pile.clone(),
            )
            .await;
            cache.insert(event.peer_id.clone(), authenticated);

            if authenticated {
                if auto_send_auth {
                    send_authentication_packet(event.peer_id.clone(), event.tx_outgoing.clone())
                        .map_err(|e| layer_err!("Failed to send authentication packet: {:?}", e))?;
                }

                let raw_packet = gen_packet(&Packet::NetworkCommandSys(SysCommandPacket {
                    sys_cmd: SysCommand::SysCommandAuth(SysAuthCommand::CompleteNotification),
                }))
                .map_err(|e| {
                    layer_err!("Failed to generate authentication success packet: {:?}", e)
                })?;

                let outgoing_packet = outgoing::OutgoingPacket {
                    peer_id: event.peer_id.clone(),
                    channel_id: 0,
                    packet: outgoing::Packet::reliable(raw_packet),
                };

                event.tx_outgoing.send(outgoing_packet).map_err(|e| {
                    layer_err!("Failed to transmit authentication success packet: {:?}", e)
                })?;
            }

            event.data_pile.insert(cache);

            return Ok(None);
        });
    }
}

fn send_authentication_packet(
    peer_id: ENetPeerID,
    tx_outgoing: Sender<outgoing::OutgoingPacket>,
) -> Result<(), String> {
    let raw_packet = gen_packet(&Packet::NetworkCommandSys(SysCommandPacket {
        sys_cmd: SysCommand::SysCommandAuth(SysAuthCommand::AuthMessage(Box::new([0x00]))),
    }))
    .map_err(|e| format!("Failed to generate authentication request packet: {:?}", e))?;

    let outgoing_packet = outgoing::OutgoingPacket {
        peer_id: peer_id.clone(),
        channel_id: 0,
        packet: outgoing::Packet::reliable(raw_packet),
    };

    tx_outgoing
        .send(outgoing_packet)
        .map_err(|e| format!("Failed to transmit authentication request packet: {:?}", e))?;

    Ok(())
}
