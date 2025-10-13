// TODO: Add Example Usage

use crate::{Layer, LayerResult, LayerReturn, event::Event};
use std::sync::Arc;

pub struct DataSyncLayer<T>
where
    T: Sync + Send + 'static,
{
    data: Arc<T>,

    func: fn(Event, Arc<T>) -> LayerResult,
}

impl<T> DataSyncLayer<T>
where
    T: Sync + Send + 'static,
{
    /// Build a [`Layer`](crate::Layer) which calls a sync function with data
    pub fn build(async_function: fn(Event, Arc<T>) -> LayerResult, data: T) -> DataSyncLayer<T> {
        DataSyncLayer {
            data: Arc::new(data),

            func: async_function,
        }
    }

    /// Build a [`Layer`](crate::Layer) which calls a sync function with data
    pub fn build_arc(
        async_function: fn(Event, Arc<T>) -> LayerResult,
        data: Arc<T>,
    ) -> DataSyncLayer<T> {
        DataSyncLayer {
            data,

            func: async_function,
        }
    }
}

impl<T> DataSyncLayer<T>
where
    T: Sync + Send + 'static + Default,
{
    pub fn build_default(async_function: fn(Event, Arc<T>) -> LayerResult) -> DataSyncLayer<T> {
        DataSyncLayer {
            data: Arc::new(T::default()),

            func: async_function,
        }
    }
}

impl<T> Layer for DataSyncLayer<T>
where
    T: Sync + Send + 'static,
{
    fn call(&self, event: Event) -> LayerReturn {
        let func = self.func;
        let data = self.data.clone();

        return Box::pin(async move { (func)(event, data) });
    }
}
