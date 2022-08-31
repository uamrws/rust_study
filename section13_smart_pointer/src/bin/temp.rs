use std::rc::Rc;
fn main() {
    // 创建Rc，持有一个值5
    let five = Rc::new(5);

    // 通过Rc，创建一个Weak指针
    let weak_five = Rc::downgrade(&five);

    // Weak引用的资源依然存在，取到值5
    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    // assert_eq!(*strong_five.unwrap(), 5);
    let b = &strong_five.unwrap();
    println!("{}", Rc::strong_count(b));
    // 手动释放资源`five`
    drop(five);
    println!("{}", Rc::strong_count(b));
    // assert_eq!(*strong_five.unwrap(), 5);
    // Weak引用的资源已不存在，因此返回None
    let pp: Option<Rc<_>> = weak_five.upgrade();
    assert_eq!(pp, None);
}

