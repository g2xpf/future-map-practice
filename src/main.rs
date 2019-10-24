extern crate async_await_test;

use async_await_test::spin_runner::SpinRunner;
// use async_await_test::spin_waker::SpinWaker;
use async_await_test::simple_future::ReturnFuture;

fn main(){
    let mut runner = SpinRunner::new();
    let future = ReturnFuture::new(42);
    let ret = runner.run(future);
    println!("answer is {}", ret);
}