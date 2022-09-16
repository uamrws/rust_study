#[derive(Debug)]
struct A<'a> {
    a: &'a str,
}

fn temp<'a, 'b, 'c>(a: &'a mut A<'b>, c: &'c str)
where
    'c: 'b,
{
    a.a = c;
}
fn temp_main<'a, 'c>(c: &'c str) {
    let mut a: A<'a> = A { a: "hello" };
    temp(&mut a, c);
}

fn main() {
    let c = "world".to_string();
    temp_main(&c);
    // let mut a = A { a: "hello" };
    // {
    //     let c = "world".to_string();
    //     temp(&mut a, &c);
    // }
}
