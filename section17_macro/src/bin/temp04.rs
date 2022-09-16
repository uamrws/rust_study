use std::{any::Any, str::FromStr};

struct B<'a> {
    b: &'a str,
}

fn main() {
    let s = String::from("asd");
    let b1 = B { b: "str" };
    let b2 = B { b: s.as_str() };
    let a: &dyn Any = &b1;
    // let a: &dyn Any = &b2;
}
