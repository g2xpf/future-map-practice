extern crate async_await_test;

use async_await_test::simple_future::ReturnFuture;
use async_await_test::spin_runner::SpinRunner;

use async_await_test::transformers::ToFutureMap;

fn main() {
    let mut runner = SpinRunner::new();
    let future = ReturnFuture::new(42)
        .map(|v| v * 2)
        .map(|v| format!("value: {}", v));
    let ret = runner.run(future);
    println!("{:?}", ret);
}
