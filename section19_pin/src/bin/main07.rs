use std::marker::PhantomPinned;
use std::pin::Pin;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let t = Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.as_ref().a;
        unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };

        boxed
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        unsafe { &*(self.b) }
    }
}

pub fn main() {
    let mut test1 = Test::new("test1");
    let mut test2 = Test::new("test2");
    let c = test1.as_mut();
    let d = unsafe { c.get_unchecked_mut() };
    d.a = String::from("test222");
    println!("{:p}", &d.a);
    d.a = String::from("test222");
    println!("{:p}", &d.a);

    println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());

    println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
}
