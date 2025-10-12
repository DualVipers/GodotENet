// TODO: Add Example Usage

use std::sync::Arc;

use crate::{Layer, LayerResult, LayerReturn, event::Event};

pub struct DataAsyncLayer<F, T>
where
    F: Future<Output = LayerResult> + Sync + Send + 'static,
    T: Sync + Send + 'static,
{
    data: Arc<T>,

    func: fn(Event, Arc<T>) -> F,
}

impl<F, T> DataAsyncLayer<F, T>
where
    F: Future<Output = LayerResult> + Sync + Send,
    T: Sync + Send + 'static,
{
    /// Build a layer which calls a async function with data
    pub fn build(async_function: fn(Event, Arc<T>) -> F, data: T) -> DataAsyncLayer<F, T> {
        DataAsyncLayer {
            data: Arc::new(data),

            func: async_function,
        }
    }

    /// Build a layer which calls a async function with data
    pub fn build_arc(async_function: fn(Event, Arc<T>) -> F, data: Arc<T>) -> DataAsyncLayer<F, T> {
        DataAsyncLayer {
            data,

            func: async_function,
        }
    }
}

impl<F, T> DataAsyncLayer<F, T>
where
    F: Future<Output = LayerResult> + Sync + Send,
    T: Sync + Send + 'static + Default,
{
    pub fn build_default(async_function: fn(Event, Arc<T>) -> F) -> DataAsyncLayer<F, T> {
        DataAsyncLayer {
            data: Arc::new(T::default()),

            func: async_function,
        }
    }
}

impl<F, T> Layer for DataAsyncLayer<F, T>
where
    F: Future<Output = LayerResult> + Sync + Send,
    T: Sync + Send + 'static,
{
    fn call(&self, event: Event) -> LayerReturn {
        let func = self.func;
        let data = self.data.clone();

        return Box::pin(async move { (func)(event, data).await });
    }
}
