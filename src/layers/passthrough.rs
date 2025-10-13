use crate::{Layer, LayerReturn, event::Event};

/// A layer which simply passes the event to the next layer.
pub struct PassthroughLayer;

impl Layer for PassthroughLayer {
    fn call(&self, event: Event) -> LayerReturn {
        return Box::pin(async move { Ok(Some(event)) });
    }
}
