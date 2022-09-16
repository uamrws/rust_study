use std::any::Any;

#[derive(Debug)]
enum E {
    H,
    He,
    Li,
}
#[derive(Debug)]
struct S {
    x: u8,
    y: u8,
    z: u16,
}
fn print_any(a: &dyn Any) {
    if let Some(x) = a.downcast_ref::<u32>() {
        println!("u32: {}", x);
    }
    if let Some(x) = a.downcast_ref::<E>() {
        println!("E: {:?}", x);
    }
    if let Some(x) = a.downcast_ref::<S>() {
        println!("S: {:?}", x);
    }
}
fn temp(b: Box<u32>) -> &u32 {
    &b
}
fn main() {
    print_any(&1_u32);
    print_any(&E::H);
    print_any(&S { x: 1, y: 2, z: 10 });
    let c = temp(Box::new(1_u32));
}
