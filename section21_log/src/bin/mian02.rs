use std::intrinsics::size_of;

trait Temp {
    fn temp();
}

struct A;
struct B;

impl Temp for A {
    fn temp() {
        println!("temp_a");
    }
}

impl Temp for B {
    fn temp() {
        println!("temp_b");
    }
}
fn temp<T>()
where
    T: Temp,
{
    T::temp();
}
fn main() {
    let a = temp::<T>();
    temp::<B>();
}
