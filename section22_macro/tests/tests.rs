use section22_macro::New;

#[derive(New)]
struct Foo;
#[derive(New)]
struct Bar {
    x: i32,
    y: String,
}

#[test]
fn test_empty_struct() {
    let foo = Foo::new(42);
}

fn test_simple_struct() {
    let bar = Bar::new(42, "hello".into());
}
