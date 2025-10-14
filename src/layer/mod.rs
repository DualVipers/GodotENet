mod async_layer;
mod data_async_layer;
mod data_sync_layer;
mod sync_layer;

pub use async_layer::*;
pub use data_async_layer::*;
pub use data_sync_layer::*;
pub use sync_layer::*;

use crate::event::Event;
use std::{error::Error as StdError, fmt::Display, future::Future, ops::Deref, pin::Pin};

pub type LayerReturn = Pin<Box<dyn Future<Output = LayerResult> + Send + Sync>>;

pub type LayerResult = Result<Option<Event>, LayerError>;

/// Layer trait for processing Godot ENet events
pub trait Layer: Send + Sync + 'static {
    /// Process a Godot ENet event
    fn call(&self, event: Event) -> LayerReturn;
}

#[derive(Debug)]
/// Error returned by a Layer
pub struct LayerError {
    message: String,

    layer: String,
}

impl LayerError {
    pub fn new(message: String, layer: String) -> Self {
        Self { message, layer }
    }
}

impl StdError for LayerError {}

impl Display for LayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.layer, self.message)
    }
}

impl Deref for LayerError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

#[macro_export]
macro_rules! layer_err {
    ($msg:expr) => {
        $crate::LayerError::new($msg.to_string(), std::any::type_name::<Self>().to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::LayerError::new(format!($fmt, $($arg)*), std::any::type_name::<Self>().to_string())
    };
    () => {
        $crate::LayerError::new("".to_string(), std::any::type_name::<Self>().to_string())
    };
}

#[macro_export]
macro_rules! fn_layer_err {
    ($layer:expr) => {
        $crate::LayerError::new("".to_string(), $layer.to_string())
    };
    ($layer:expr, $msg:expr) => {
        $crate::LayerError::new($msg.to_string(), $layer.to_string())
    };
    ($layer:expr, $fmt:expr, $($arg:tt)*) => {
        $crate::LayerError::new(format!($fmt, $($arg)*), $layer.to_string())
    };
}
