use std::{thread::sleep, time::Duration};

use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

async fn hello_cat() {
    sleep(Duration::from_secs(1));
    println!("hello, kitty!");
}

async fn async_main() {
    let f1 = hello_world();
    let f2 = hello_cat();

    futures::join!(f2, f1,);
}

fn main() {
    block_on(async_main());
}
