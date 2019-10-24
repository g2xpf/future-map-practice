use std::pin::Pin;
use std::future::Future;
use std::task::{Poll, Context};

pub struct ReturnFuture<T>(Option<T>);

impl<T> ReturnFuture<T> {
    pub fn new(t: T) -> Self {
        Self(Some(t))
    }
}

impl<T> Unpin for ReturnFuture<T> {}

impl<T> Future for ReturnFuture<T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, _: &mut Context) -> Poll<Self::Output> {
        Poll::Ready(
            self.get_mut()
                .0
                .take()
                .expect("A future should never be polled after it returns Ready"),
        )
    }
}
