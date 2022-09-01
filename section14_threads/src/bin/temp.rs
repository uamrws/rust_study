use std::sync::Arc;
use std::thread;

fn main() {
    let s = Arc::new(String::from("多线程漫游者"));
    for _ in 0..10 {
        let s = Arc::clone(&s);

        let handle = thread::spawn(move || {
            s.clone();
            println!("{}", s)
        });
    }
}
