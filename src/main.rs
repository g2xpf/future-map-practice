extern crate future_map_practice;

use future_map_practice::simple_future::ReturnFuture;
use future_map_practice::spin_runner::SpinRunner;

use future_map_practice::transformers::ToFutureMap;

fn main() {
    let mut runner = SpinRunner::new();
    let future = ReturnFuture::new(42)
        .map(|v| {
            std::thread::sleep(std::time::Duration::from_millis(3000));
            v * 2
        })
        .map(|v| format!("value: {}", v));
    let ret = runner.run(future);
    println!("{:?}", ret);
}
