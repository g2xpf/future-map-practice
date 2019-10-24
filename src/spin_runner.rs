use super::spin_waker::SpinWaker;
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct SpinRunner;

impl SpinRunner {
    pub fn new() -> Self {
        Self
    }

    pub fn run<F>(&mut self, future: F) -> F::Output
    where
        F: Future,
    {
        let mut futures: Pin<Box<_>> = Box::new(future).into();
        self.run_pin(futures.as_mut())
    }

    pub fn run_pin<F>(&mut self, mut future: Pin<F>) -> <<F as Deref>::Target as Future>::Output
    where
        F: DerefMut,
        <F as Deref>::Target: Future,
    {
        let waker = SpinWaker::waker();
        let mut cx = Context::from_waker(&waker);

        loop {
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(ret) => {
                    return ret;
                }
                Poll::Pending => {
                    std::thread::sleep(std::time::Duration::from_millis(4));
                }
            }
        }
    }
}
