use crate::{
    GDPeerID, Layer, LayerResult,
    event::{Event, EventType},
    packet::{Packet, RemoteCacheID, confirm_path, outgoing},
};
use dashmap::DashMap;
use log::{debug, error};
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct PathCache {
    id_cache: Arc<DashMap<GDPeerID, DashMap<RemoteCacheID, String>>>,
    path_cache: Arc<DashMap<GDPeerID, DashMap<String, RemoteCacheID>>>,
    checksum_cache: Arc<DashMap<GDPeerID, DashMap<RemoteCacheID, String>>>,
}

impl PathCache {
    pub fn create_cache_entry(&self, gd_peer: &GDPeerID) {
        self.path_cache.insert(*gd_peer, DashMap::new());
        self.id_cache.insert(*gd_peer, DashMap::new());
        self.checksum_cache.insert(*gd_peer, DashMap::new());

        debug!("Created path cache entry for Godot Peer ID: {:?}", gd_peer);
    }

    pub fn remove_cache_entry(&self, gd_peer: &GDPeerID) {
        self.path_cache.remove(gd_peer);
        self.id_cache.remove(gd_peer);
        self.checksum_cache.remove(gd_peer);

        debug!("Removed path cache entry for Godot Peer ID: {:?}", gd_peer);
    }

    pub fn get_path(&self, peer: &GDPeerID, remote_cache_id: &RemoteCacheID) -> Option<String> {
        let id_cache = self.id_cache.get(peer)?;

        id_cache
            .get(remote_cache_id)
            .map(|entry| entry.value().clone())
    }

    pub fn get_id(&self, peer: &GDPeerID, path: &str) -> Option<RemoteCacheID> {
        let path_cache = self.path_cache.get(peer)?;

        path_cache.get(path).map(|entry| *entry.value())
    }

    pub fn get_checksum(&self, peer: &GDPeerID, remote_cache_id: &RemoteCacheID) -> Option<String> {
        let checksum_cache = self.checksum_cache.get(peer)?;

        checksum_cache
            .get(remote_cache_id)
            .map(|entry| entry.value().clone())
    }

    pub fn insert(
        &self,
        peer: &GDPeerID,
        remote_cache_id: RemoteCacheID,
        path: &str,
        checksum: &str,
    ) -> Result<(), String> {
        let path_cache = self.path_cache.get(peer);
        let id_cache = self.id_cache.get(peer);
        let checksum_cache = self.checksum_cache.get(peer);

        if let (Some(path_cache), Some(id_cache), Some(checksum_cache)) =
            (path_cache, id_cache, checksum_cache)
        {
            path_cache.insert(path.to_string(), remote_cache_id);
            id_cache.insert(remote_cache_id, path.to_string());
            checksum_cache.insert(remote_cache_id, checksum.to_string());

            debug!(
                "Inserted path cache entry for Godot Peer ID: {:?}, Remote Cache ID: {}, Path: {}",
                peer, remote_cache_id, path
            );
        } else {
            return Err(format!(
                "Failed to insert path cache entry for Godot Peer ID: {:?}, Remote Cache ID: {}, Path: {} - No cache entry found for peer",
                peer, remote_cache_id, path
            ));
        }

        return Ok(());
    }

    pub fn remove_id(&self, peer: &GDPeerID, remote_cache_id: &RemoteCacheID) {
        let id_cache = self.id_cache.get(peer);
        let path_cache = self.path_cache.get(peer);
        let checksum_cache = self.checksum_cache.get(peer);

        if let (Some(id_cache), Some(path_cache), Some(checksum_cache)) =
            (id_cache, path_cache, checksum_cache)
        {
            if let Some(path) = id_cache.remove(remote_cache_id) {
                path_cache.remove(&path.1);
                checksum_cache.remove(remote_cache_id);

                debug!(
                    "Removed path cache entry for Godot Peer ID: {:?}, Remote Cache ID: {}, Path: {}",
                    peer, remote_cache_id, path.1
                );
            }
        } else {
            error!(
                "Failed to remove path cache entry for Godot Peer ID: {:?}, Remote Cache ID: {} - No cache entry found for peer",
                peer, remote_cache_id
            );
        }
    }

    pub fn remove_path(&self, peer: &GDPeerID, path: &str) {
        let path_cache = self.path_cache.get(peer);
        let id_cache = self.id_cache.get(peer);
        let checksum_cache = self.checksum_cache.get(peer);

        if let (Some(path_cache), Some(id_cache), Some(checksum_cache)) =
            (path_cache, id_cache, checksum_cache)
        {
            if let Some(remote_cache_id) = path_cache.remove(path) {
                id_cache.remove(&remote_cache_id.1);
                checksum_cache.remove(&remote_cache_id.1);

                debug!(
                    "Removed path cache entry for Godot Peer ID: {:?}, Remote Cache ID: {}, Path: {}",
                    peer, remote_cache_id.1, path
                );
            }
        } else {
            error!(
                "Failed to remove path cache entry for Godot Peer ID: {:?}, Path: {} - No cache entry found for peer",
                peer, path
            );
        }
    }
}

#[derive(Clone)]
/// A layer which maintains a path cache for each id, adding the cache to the DataPile.
///
/// Depends on [`super::AutoParseLayer`] and [`super::PeerMapLayer`].
pub struct PathCacheLayer {
    cache: PathCache,

    consume_confirm_path: bool,
    consume_simplify_path: bool,
}

impl Default for PathCacheLayer {
    fn default() -> Self {
        Self {
            cache: PathCache::default(),

            consume_confirm_path: true,
            consume_simplify_path: true,
        }
    }
}

impl Layer for PathCacheLayer {
    fn call(
        &self,
        mut event: Event,
    ) -> std::pin::Pin<Box<dyn Future<Output = LayerResult> + Send + Sync + 'static>> {
        let cache = self.cache.clone();
        let consume_confirm_path = self.consume_confirm_path;
        let consume_simplify_path = self.consume_simplify_path;

        return Box::pin(async move {
            match event.event {
                EventType::Connect { ref godot_peer } => {
                    cache.create_cache_entry(godot_peer);
                }
                EventType::Disconnect { ref godot_peer } => {
                    cache.remove_cache_entry(godot_peer);
                }
                EventType::Receive { .. } => {
                    let parsed_packet = match event.data_pile.get::<Packet>() {
                        Some(packet) => packet,
                        None => {
                            return Err(
                                "PathCacheLayer ran without parsed packet, requires AutoParseLayer"
                                    .to_string(),
                            );
                        }
                    };

                    let peer_id = match event.data_pile.get::<GDPeerID>() {
                        Some(peer_id) => peer_id,
                        None => {
                            return Err("PathCacheLayer ran without Godot Peer ID in DataPile, requires PeerMapLayer".to_string());
                        }
                    };

                    if let Packet::NetworkCommandConfirmPath {
                        remote_cache_id, ..
                    } = parsed_packet
                    {
                        debug!(
                            "PathCacheLayer received ConfirmPath packet for Godot Peer {:?} with Remote Cache ID: {:?}",
                            peer_id, remote_cache_id
                        );

                        if consume_confirm_path {
                            return Ok(None);
                        }
                    } else if let Packet::NetworkCommandSimplifyPath {
                        remote_cache_id,
                        path,
                        methods_md5_hash,
                    } = parsed_packet
                    {
                        debug!(
                            "PathCacheLayer received SimplifyPath packet for Godot Peer {:?} with Remote Cache ID: {:?}, Path: {}",
                            peer_id, remote_cache_id, path
                        );

                        if let Err(e) =
                            cache.insert(peer_id, *remote_cache_id, path, methods_md5_hash)
                        {
                            return Err(format!(
                                "Failed to insert into path cache for Godot Peer {:?}: \n{}",
                                peer_id, e
                            ));
                        }

                        let response_packet: Vec<u8> = match confirm_path::gen_packet(
                            true,
                            *remote_cache_id,
                        ) {
                            Ok(packet) => packet,
                            Err(e) => {
                                return Err(format!(
                                    "Failed to generate ConfirmPath response packet for Godot Peer {:?}, Remote Cache ID: {}: \n{}",
                                    peer_id, remote_cache_id, e,
                                ));
                            }
                        };

                        let outgoing_packet = outgoing::OutgoingPacket {
                            peer_id: event.peer_id,
                            channel_id: 0,
                            packet: outgoing::Packet::reliable(response_packet),
                        };

                        if let Err(e) = event.tx_outgoing.send(outgoing_packet) {
                            return Err(format!("Failed to send ConfirmPath packet: {:?}", e));
                        }

                        if consume_simplify_path {
                            return Ok(None);
                        }
                    }
                }
            }

            event.data_pile.insert(cache);

            return Ok(Some(event));
        });
    }
}
