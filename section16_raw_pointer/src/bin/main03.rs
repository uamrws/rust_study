#![allow(unused)]
fn main() {
    #[repr(C)]
    union MyUnion<'a> {
        f1: u8,
        f2: &'a str,
    }
    let mut a = MyUnion { f1: 1 };
    println!("{:?}", unsafe { a.f2 });
    a.f2 = "2";
    <[_]>::into_vec(Box::new([1, 2, 3, 4]));
}
