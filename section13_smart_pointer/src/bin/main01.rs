fn display(s: &str) {
    println!("{}", s);
}
fn main() {
    let mut m = MyBox::new(String::from("Rust"));
    (*m).push_str("string");
    display(&m);
}
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
use std::ops::{Deref, DerefMut};

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
