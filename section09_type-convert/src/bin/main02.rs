trait A {
    fn temp(&self);
}
struct B;
impl A for B {
    fn temp(&self) {
        println!("{}", 1)
    }
}
impl A for &B {
    fn temp(&self) {
        println!("{}", 2)
    }
}
fn main() {
    let b = &B {};
    b.temp();
}
