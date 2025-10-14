use crate::{
    Layer, LayerReturn,
    event::Event,
    layer_err,
    packet::{Packet, rpc::RPCCommand},
};
use dashmap::DashMap;
use std::sync::Arc;

/// A routing layer which redirects the request to the path based on the saved path's name.
///
/// Depends on [`RPCParseLayer`](crate::layers::RPCParseLayer)
pub struct RPCPathRouter {
    paths_cache: Arc<DashMap<String, Arc<dyn Layer>>>,
}

// Todo: Look to other routers to make creation easier
impl RPCPathRouter {
    pub fn new() -> RPCPathRouter {
        RPCPathRouter {
            paths_cache: Arc::new(DashMap::new()),
        }
    }

    pub fn register_path(&self, path: String, layer: Arc<dyn Layer>) {
        self.paths_cache.insert(path, layer);
    }
}

impl Layer for RPCPathRouter {
    fn call(&self, event: Event) -> LayerReturn {
        let cache = self.paths_cache.clone();

        return Box::pin(async move {
            let Some(packet) = event.data_pile.get::<Packet>() else {
                return Ok(Some(event));
            };

            if !matches!(*packet, Packet::NetworkCommandRemoteCall(_)) {
                return Ok(Some(event));
            }

            if let Some(command) = event.data_pile.get::<RPCCommand>() {
                let path = command.path.clone();

                if let Some(layer) = cache.get(&path) {
                    return layer.call(event).await;
                } else {
                    return Ok(Some(event));
                }
            } else {
                return Err(layer_err!(
                    "Ran without parsed packet, requires RPCParseLayer".to_string()
                ));
            }
        });
    }
}
