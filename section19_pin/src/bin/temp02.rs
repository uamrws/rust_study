#![allow(unused)]
use std::marker::PhantomPinned;
use std::pin::Pin;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned, // 这个标记可以让我们的类型自动实现特征`!Unpin`
        }
    }

    fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr;
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> *const String {
        assert!(
            !self.b.is_null(),
            "Test::b called without Test::init being called first"
        );
        self.b
    }
}
fn main() {
    // 此时的`test1`可以被安全的移动
    let mut test1 = Test::new("test1");
    // 新的`test1`由于使用了`Pin`，因此无法再被移动，这里的声明会将之前的`test1`遮蔽掉(shadow)
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1.as_mut());

    let mut test2 = Test::new("test2");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
    Test::init(test2.as_mut());

    println!(
        "a: {:p}, b: {:?}",
        Test::a(test2.as_ref()),
        Test::b(test2.as_ref())
    );
    let this = unsafe { test2.as_mut().get_unchecked_mut() };
    for i in 1..100000 {
        this.a.push_str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaassssssssssssssssssssdddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd");
        this.a.push_str("string");
    }
    assert_eq!(Test::a(test2.as_ref()), unsafe {
        &*Test::b(test2.as_ref())
    });
    println!(
        "a: {:p}, b: {:?}",
        Test::a(test2.as_ref()),
        Test::b(test2.as_ref())
    );
}s
