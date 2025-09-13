mod async_layer;
mod sync_layer;

pub use async_layer::*;
pub use sync_layer::*;

use crate::event::Event;
use std::{future::Future, pin::Pin};

pub type LayerReturn = Pin<Box<dyn Future<Output = LayerResult> + Send + Sync>>;

pub type LayerResult = Result<Option<Event>, String>;

/// Layer trait for processing Godot ENet events
pub trait Layer {
    /// Process a Godot ENet event
    fn call(&self, event: Event) -> LayerReturn;
}
