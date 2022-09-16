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

fn print_any(a: Box<dyn Any>) {
    if let Some(x) = a.downcast_ref::<u32>() {
        println!("u32: {}", x);
    }
    // if let Ok(x) = a.downcast::<E>() {
    //     println!("E: {:?}", x);
    // }
    // if let Ok(x) = a.downcast::<S>() {
    //     println!("S: {:?}", x);
    // }
}
fn main() {
    print_any(Box::new(1_u32));
    print_any(Box::new(E::H));
    print_any(Box::new(S { x: 1, y: 2, z: 10 }));
}
