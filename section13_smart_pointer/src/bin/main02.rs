use std::sync::Arc;
use std::thread::{self};

fn main() {
    let s = Arc::new(String::from("多线程漫游者"));
    let mut a: Vec<_> = vec![];
    for _ in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || println!("{}", s));
        a.push(handle);
    }
    for i in a {
        _ = i.join();
    }
}
