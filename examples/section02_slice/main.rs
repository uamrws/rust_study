fn main() {
    let s = String::from("hello world");
    let a = &s;
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{a} is {hello}, {world}")
}
