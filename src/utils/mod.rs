use crate::error::Error;
use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct JsonStream<T> {
    inner: Pin<Box<dyn Stream<Item = Result<T, Error>>>>,
}

impl<T> JsonStream<T> {
    pub fn new<S>(stream: S) -> Self
    where
        S: Stream<Item = Result<T, Error>> + 'static,
    {
        Self {
            inner: Box::pin(stream),
        }
    }
}

impl<T> Stream for JsonStream<T> {
    type Item = Result<T, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.as_mut().poll_next(cx)
    }
}
