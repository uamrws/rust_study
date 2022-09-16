#[derive(Debug)]
struct A<'a> {
    a: &'a str,
}

fn temp<'b, 'c>(a: &mut A<'b>, c: &'c str)
where
    'c: 'b,
{
    a.a = c;
}
fn temp_main<'a>() {
    let mut a: A<'a> = A { a: "hello" };
    {
        let c = "world".to_string();
        temp(&mut a, &c);
    }
}

fn main() {
    temp_main();
    // let mut a = A { a: "hello" };
    // {
    //     let c = "world".to_string();
    //     temp(&mut a, &c);
    // }
}
