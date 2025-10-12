// TODO: Add Example Usage

use crate::{Layer, LayerResult, LayerReturn, event::Event};

pub struct SyncLayer {
    func: fn(Event) -> LayerResult,
}

impl SyncLayer {
    /// Build a layer which calls a sync function
    pub fn build(async_function: fn(Event) -> LayerResult) -> SyncLayer {
        SyncLayer {
            func: async_function,
        }
    }
}

impl Layer for SyncLayer {
    fn call(&self, event: Event) -> LayerReturn {
        let func = self.func;

        return Box::pin(async move { (func)(event) });
    }
}
