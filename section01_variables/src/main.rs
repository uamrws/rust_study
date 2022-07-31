fn main() {
    let mut x: i8 = 127;
    println!("The value of x is: {x:b}");
    x = 6;
    println!("The value of x is: {x}");
    let _guess: usize = "42".parse().expect("Not a number!");
    let _first_letter = 'A';
    let _space = ' '; // A space inside ' ' is also a char
    let _other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
    let _cat_face = '😺'; // Emojis are chars too

    let number = 32;
    println!("{}", number as u8 as char);

    // 元祖tup
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let _one = x.2;
    println!("{}", 1);

    let (a, b, c, d);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];

    assert_eq!([1, 2, 1, 4], [a, b, c, d]);
}
