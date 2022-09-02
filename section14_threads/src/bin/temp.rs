static mut A: String = String::new();
fn main() {
    unsafe {
        A.push_str("asd");
        println!("{}", A)
    }
}
