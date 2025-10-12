use crate::{Layer, LayerReturn, event::Event, packet::Packet};
use dashmap::DashMap;
use std::sync::Arc;

/// A routing layer which redirects the request to the path based on the function id.
pub struct RPCFunctionNameRouter {
    function_name_cache: Arc<DashMap<u32, Arc<dyn Layer>>>,
}

// Todo: Look to other routers to make creation easier
impl RPCFunctionNameRouter {
    pub fn new() -> RPCFunctionNameRouter {
        RPCFunctionNameRouter {
            function_name_cache: Arc::new(DashMap::new()),
        }
    }

    pub fn register_name_id(&self, id: u32, layer: Arc<dyn Layer>) {
        self.function_name_cache.insert(id, layer);
    }
}

impl Layer for RPCFunctionNameRouter {
    fn call(&self, event: Event) -> LayerReturn {
        let cache = self.function_name_cache.clone();

        return Box::pin(async move {
            let Some(packet) = event.data_pile.get::<Packet>() else {
                return Ok(Some(event));
            };

            let Packet::NetworkCommandRemoteCall(header) = packet else {
                return Ok(Some(event));
            };

            if let Some(layer) = cache.get(&header.name_id) {
                return layer.call(event).await;
            } else {
                return Ok(Some(event));
            }
        });
    }
}
