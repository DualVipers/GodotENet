// TODO: Add Example Usage

use crate::{Layer, LayerResult, LayerReturn, event::Event};

pub struct AsyncLayer<F>
where
    F: Future<Output = LayerResult> + Sync + Send + 'static,
{
    func: fn(Event) -> F,
}

impl<F> AsyncLayer<F>
where
    F: Future<Output = LayerResult> + Sync + Send,
{
    /// Build a [`Layer`](crate::Layer) which calls an async function
    pub fn build(async_function: fn(Event) -> F) -> AsyncLayer<F> {
        AsyncLayer {
            func: async_function,
        }
    }
}

impl<F> Layer for AsyncLayer<F>
where
    F: Future<Output = LayerResult> + Sync + Send,
{
    fn call(&self, event: Event) -> LayerReturn {
        let func = self.func;

        return Box::pin(async move { (func)(event).await });
    }
}
