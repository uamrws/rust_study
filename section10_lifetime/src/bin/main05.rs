use std::result;

fn main() {
    let result;
    let string1 = String::from("abcd");
    {
        let string2 = "xyz";
        result = longest(string1.as_str(), string2);
    }
    println!("The longest string is {}", result);
}

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'b str {
    x
}
