#![allow(unused)]
fn main() {
    // &'a mut &'b 两个引用时，编译器会默认'b: 'a
    fn temp<'a, 'b>(s: &'b mut &'a str) -> &'b mut &'b str {
        s
    }
    // 可以通过编译
    // fn temp<'a, 'b>(s: &'a mut &'a str) -> &'a mut &'a str
    // fn temp<'a, 'b>(s: &'a mut &'a str) -> &'b mut &'a str
    //
    // fn temp<'a, 'b>(s: &'a mut &'b str) -> &'a mut &'b str
    // fn temp<'a, 'b>(s: &'a mut &'b str) -> &'b mut &'a str
    //
    // fn temp<'a, 'b>(s: &'b mut &'a str) -> &'a mut &'b str
    // fn temp<'a, 'b>(s: &'b mut &'a str) -> &'b mut &'a str
    //
    // fn temp<'a, 'b>(s: &'b mut &'b str) -> &'a mut &'b str
    // fn temp<'a, 'b>(s: &'b mut &'b str) -> &'b mut &'a str
    // -----------------------------------------------------
    // 无法通过编译
    // fn temp<'a, 'b>(s: &'a mut &'a str) -> &'a mut &'b str
    // fn temp<'a, 'b>(s: &'a mut &'a str) -> &'b mut &'b str
    //
    // fn temp<'a, 'b>(s: &'a mut &'b str) -> &'a mut &'a str
    // fn temp<'a, 'b>(s: &'a mut &'b str) -> &'b mut &'b str
    //
    // fn temp<'a, 'b>(s: &'b mut &'a str) -> &'a mut &'a str
    // fn temp<'a, 'b>(s: &'b mut &'a str) -> &'b mut &'b str
    //
    // fn temp<'a, 'b>(s: &'b mut &'b str) -> &'a mut &'a str
    // fn temp<'a, 'b>(s: &'b mut &'b str) -> &'b mut &'b str
}
