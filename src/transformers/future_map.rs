use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;

pub struct FutureMap<F, M> {
    future: F,
    map_fn: Option<M>
}

pub trait ToFutureMap: Future {
    fn map<M, U>(self, map_fn: M) -> FutureMap<Self, M>
    where
        Self: Future + Sized + Unpin,
        M: Unpin + FnOnce(Self::Output) -> U {
            FutureMap::new(self, map_fn)
    }
}

impl<F: Future> ToFutureMap for F {}

impl<F, M, U> FutureMap<F, M> 
where
    F: Future + Unpin,
    M: Unpin + FnOnce(<F as Future>::Output) -> U
    {
    fn new(future: F, map_fn: M) -> FutureMap<F, M> {
        let map_fn = Some(map_fn);
        FutureMap {
            future, map_fn
        }
    }
}

impl<F, M, U> Future for FutureMap<F, M>
where
    F: Future + Unpin,
    M: Unpin + FnOnce(<F as Future>::Output) -> U
    {
    type Output = U;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let mut map_fn = None;
        let mut future = unsafe { 
            self.map_unchecked_mut(|this| {
                map_fn = Some(this.map_fn.take().unwrap());
                &mut this.future
            })
        };
        
        match future.as_mut().poll(ctx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(v) => Poll::Ready((map_fn.unwrap())(v))
        }
    }
}