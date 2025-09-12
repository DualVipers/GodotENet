//TODO: REMOVE FOLLOWING IMPORT
pub mod testing;

// TODO: Async and Sync Layer Variants?

use crate::event::GodotENetEvent;
use std::{future::Future, pin::Pin};

pub type GodotENetLayerReturn = Pin<Box<dyn Future<Output = GodotENetLayerResult> + Send + Sync>>;

pub type GodotENetLayerResult = Result<Option<GodotENetEvent>, String>;

/// Layer trait for processing Godot ENet events
pub trait GodotENetLayer {
    /// Process a Godot ENet event
    fn call(
        &self,
        event: GodotENetEvent,
    ) -> Pin<Box<dyn Future<Output = GodotENetLayerResult> + Send + Sync>>;
}
