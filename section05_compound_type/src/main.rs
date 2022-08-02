fn main() {
    let mut s = String::from("hello world");
    let s1 = s.repeat(2);

    let word = first_word(&s);

    // s.clear(); // error!

    println!("the first word is: {}", word);
}
fn first_word(s: &String) -> &str {
    &s[..1]
}
