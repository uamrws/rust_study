use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct Owner {
    name: String,
    gadgets: RefCell<Vec<Weak<Gadget>>>,
}
struct Gadget {
    name: String,
    owner: Rc<Owner>,
}
fn main() {
    let o = Rc::new(Owner {
        name: "a".to_string(),
        gadgets: RefCell::new(Vec::new()),
    });
    let gadget1 = Rc::new(Gadget {
        name: "a".to_string(),
        owner: Rc::clone(&o),
    });
    let gadget2 = Rc::new(Gadget {
        name: "b".to_string(),
        owner: Rc::clone(&o),
    });

    o.gadgets
        .borrow_mut()
        .append(&mut vec![Rc::downgrade(&gadget1), Rc::downgrade(&gadget2)]);
    for gadget_opt in o.gadgets.borrow().iter() {
        // gadget_opt 是一个 Weak<Gadget> 。 因为 weak 指针不能保证他所引用的对象
        // 仍然存在。所以我们需要显式的调用 upgrade() 来通过其返回值(Option<_>)来判
        // 断其所指向的对象是否存在。
        // 当然，Option 为 None 的时候这个引用原对象就不存在了。
        let gadget = gadget_opt.upgrade().unwrap();
        println!(
            "Gadget {} owned by {}",
            gadget.name,
            gadget.owner.name
        );
    }
}
