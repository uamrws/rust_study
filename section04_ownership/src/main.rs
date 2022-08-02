#![allow(unused)]
fn main() {
    let mut s = String::from("hello");
    let r1: &mut String = &mut "1".to_string();

    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
    println!("{}", r1);
    let r2 = &mut s;
    println!("{}", r##"as"d"#"##);

    let ss: Box<str> = "hello, world".into();
    println!("{}", ss);

    let arr = &[1, 2, 3];
    let s1 = *arr;

    let s2: &str = "hello, world";

    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

}
