#![allow(unused)]
fn main() {
    fn foo() -> i32 {
        0
    }

    let pointer = foo as *const ();
    let function = unsafe {
        // 将裸指针转换为函数指针
        std::mem::transmute::<*const (), fn() -> i64>(pointer)
    };
    assert_eq!(function(), 0_i64);
}
