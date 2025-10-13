use crate::{Layer, LayerReturn, event::Event};

/// A [`Layer`](crate::Layer) which simply passes the event to the next [`Layer`](crate::Layer).
pub struct PassthroughLayer;

impl Layer for PassthroughLayer {
    fn call(&self, event: Event) -> LayerReturn {
        return Box::pin(async move { Ok(Some(event)) });
    }
}
