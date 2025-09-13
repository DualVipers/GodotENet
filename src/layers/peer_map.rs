use std::sync::Arc;

use crate::{
    GDPeerID, Layer, LayerResult,
    event::{Event, EventType},
};
use dashmap::DashMap;
use log::debug;
use rusty_enet::PeerID;

#[derive(Default, Clone)]
pub struct PeerMap {
    enet_peers: Arc<DashMap<PeerID, GDPeerID>>,
    gd_peers: Arc<DashMap<GDPeerID, PeerID>>,
}

impl PeerMap {
    pub fn get_enet_peer(&self, gd_peer: &GDPeerID) -> Option<PeerID> {
        return self.gd_peers.get(gd_peer).map(|entry| *entry.value());
    }

    pub fn get_gd_peer(&self, enet_peer: &PeerID) -> Option<GDPeerID> {
        return self.enet_peers.get(enet_peer).map(|entry| *entry.value());
    }

    pub fn insert(&self, enet_peer: PeerID, gd_peer: GDPeerID) {
        self.enet_peers.insert(enet_peer, gd_peer);
        self.gd_peers.insert(gd_peer, enet_peer);
    }

    pub fn remove_enet_peer(&self, enet_peer: &PeerID) {
        if let Some((_, gd_peer)) = self.enet_peers.remove(enet_peer) {
            self.gd_peers.remove(&gd_peer);
        }
    }

    pub fn remove_gd_peer(&self, gd_peer: &GDPeerID) {
        if let Some((_, enet_peer)) = self.gd_peers.remove(gd_peer) {
            self.enet_peers.remove(&enet_peer);
        }
    }
}

#[derive(Default, Clone)]
pub struct PeerMapLayer {
    peer_map: PeerMap,
}

impl Layer for PeerMapLayer {
    fn call(
        &self,
        mut event: Event,
    ) -> std::pin::Pin<Box<dyn Future<Output = LayerResult> + Send + Sync>> {
        let peer_map = self.peer_map.clone();

        return Box::pin(async move {
            match event.event {
                EventType::Connect { ref godot_peer } => {
                    debug!(
                        "Godot Peer Connected: {:?}\nOn ENet Peer: {:?}",
                        godot_peer, event.peer_id
                    );

                    peer_map.insert(event.peer_id, *godot_peer);
                }
                EventType::Disconnect { ref godot_peer } => {
                    debug!(
                        "Godot Peer Disconnected: {:?}\nOn ENet Peer: {:?}",
                        godot_peer, event.peer_id
                    );

                    peer_map.remove_gd_peer(godot_peer);
                }
                _ => {}
            }

            event.data_pile.insert(peer_map);

            return Ok(Some(event));
        });
    }
}
