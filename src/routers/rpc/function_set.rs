use crate::{GDPeerID, Layer, LayerReturn, event::Event, layers::PathCache, packet::Packet};
use dashmap::DashMap;
use log::debug;
use std::sync::Arc;

/// A routing layer which redirects the request to the path based on the saved path's name.
///
/// Depends on [`PathCacheLayer`](crate::layers::PathCacheLayer)
pub struct RPCFunctionSetRouter {
    function_set_cache: Arc<DashMap<String, Arc<dyn Layer>>>,
}

// Todo: Look to other routers to make creation easier
impl RPCFunctionSetRouter {
    pub fn new() -> RPCFunctionSetRouter {
        RPCFunctionSetRouter {
            function_set_cache: Arc::new(DashMap::new()),
        }
    }

    pub fn register_function_set(&self, names: &[String], layer: Arc<dyn Layer>) {
        self.function_set_cache
            .insert(super::hash_function_set(names), layer);
    }

    pub fn register_hash(&self, hash: String, layer: Arc<dyn Layer>) {
        self.function_set_cache.insert(hash, layer);
    }
}

impl Layer for RPCFunctionSetRouter {
    fn call(&self, event: Event) -> LayerReturn {
        let cache = self.function_set_cache.clone();

        return Box::pin(async move {
            let Some(packet) = event.data_pile.get::<Packet>() else {
                return Ok(Some(event));
            };

            let Packet::NetworkCommandRemoteCall(header) = packet else {
                return Ok(Some(event));
            };

            if let Some(path_cache) = event.data_pile.get::<PathCache>()
                && let Some(peer) = event.data_pile.get::<GDPeerID>()
            {
                let Some(entry) = path_cache.get_checksum(peer, &header.node_id) else {
                    debug!(
                        "RPCFunctionSetRouter could not find path for node id: {}\nfor peer: {:?}",
                        header.node_id, *peer
                    );

                    return Ok(Some(event));
                };

                if let Some(layer) = cache.get(&entry) {
                    return layer.call(event).await;
                } else {
                    return Ok(Some(event));
                }
            } else {
                return Err(
                    "RPCFunctionSetRouter ran without Path Cache or GDPeerID, requires PathCacheLayer"
                        .to_string(),
                );
            }
        });
    }
}
