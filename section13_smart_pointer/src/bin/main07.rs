use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct SelfRef {
    value: Rc<RefCell<String>>,
    pointer_to_value: Weak<RefCell<String>>,
}

impl SelfRef {
    fn new(txt: &str) -> Self {
        let s = Rc::new(RefCell::new(String::from(txt)));
        SelfRef {
            pointer_to_value: Rc::downgrade(&s),
            value: s,
        }
    }
}

fn main() {
    let t = SelfRef::new("hello");
    // 打印值和指针地址
    println!(
        "{:?}, {:?}",
        t.value.borrow(),
        t.pointer_to_value.upgrade().unwrap().borrow()
    );
}
