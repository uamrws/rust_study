use std::cell::RefCell;

fn main() {
    let s = RefCell::new(String::from("hello, world"));

    let mut s2 = s.borrow_mut();
    s2.push_str("string");

    println!("{}", s2);
    let mut b = Box::new(String::from("hello, world"));
    b.push_str("string");
    println!("{}", b)
    
}
