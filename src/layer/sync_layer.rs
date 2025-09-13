// TODO: Add Example Usage

use std::pin::Pin;

use crate::{Layer, LayerResult, event::Event};

pub struct SyncLayer {
    func: fn(Event) -> LayerResult,
}

impl SyncLayer {
    pub fn build(async_function: fn(Event) -> LayerResult) -> SyncLayer {
        SyncLayer {
            func: async_function,
        }
    }
}

impl Layer for SyncLayer {
    fn call(
        &self,
        event: Event,
    ) -> Pin<Box<(dyn Future<Output = Result<Option<Event>, String>> + Send + Sync + 'static)>>
    {
        let func = self.func;

        return Box::pin(async move { (func)(event) });
    }
}
