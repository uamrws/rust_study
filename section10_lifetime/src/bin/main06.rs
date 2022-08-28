use std::{slice::from_raw_parts, str::from_utf8_unchecked};

fn main() {
    let d;
    {
        let a = String::from("asd");

        let s = &a;
        let p = s.as_ptr() as usize;
        let l = s.len();
        d = get_str_at_location(p, l);
    }
    println!("{d}");
}
fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    // 使用裸指针需要 `unsafe{}` 语句块
    unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}
