use std::sync::Arc;

use crate::{
    ENetPeerID, GDPeerID, Layer, LayerReturn,
    event::{Event, EventType},
};
use dashmap::DashMap;
use log::debug;

#[derive(Default, Clone)]
pub struct PeerMap {
    enet_peers: Arc<DashMap<ENetPeerID, GDPeerID>>,
    gd_peers: Arc<DashMap<GDPeerID, ENetPeerID>>,
}

impl PeerMap {
    pub fn get_enet_peer(&self, gd_peer: &GDPeerID) -> Option<ENetPeerID> {
        return self.gd_peers.get(gd_peer).map(|entry| *entry.value());
    }

    pub fn get_gd_peer(&self, enet_peer: &ENetPeerID) -> Option<GDPeerID> {
        return self.enet_peers.get(enet_peer).map(|entry| *entry.value());
    }

    pub fn insert(&self, enet_peer: ENetPeerID, gd_peer: GDPeerID) {
        self.enet_peers.insert(enet_peer, gd_peer);
        self.gd_peers.insert(gd_peer, enet_peer);
    }

    pub fn remove_enet_peer(&self, enet_peer: &ENetPeerID) {
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
/// A [`Layer`](crate::Layer) which maintains a mapping between ENet PeerIDs and Godot PeerIDs,
/// adding the mapping to the [`Datapile`](crate::DataPile).
pub struct PeerMapLayer {
    peer_map: PeerMap,
}

impl Layer for PeerMapLayer {
    fn call(&self, mut event: Event) -> LayerReturn {
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

            event.data_pile.insert(event.peer_id);

            let godot_peer_id = peer_map.get_gd_peer(&event.peer_id);
            if let Some(godot_peer_id) = godot_peer_id {
                event.data_pile.insert(godot_peer_id);
            } else {
                return Err(format!(
                    "PeerMapLayer could not find Godot Peer ID for ENet Peer ID: {:?}",
                    event.peer_id
                ));
            }

            event.data_pile.insert(peer_map);

            return Ok(Some(event));
        });
    }
}
