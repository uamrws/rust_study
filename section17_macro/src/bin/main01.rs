#![allow(unused)]
fn main() {
    mod macros {
        #[macro_export]
        macro_rules! temp_vec {
            ( $( $x:expr);+ $(;)? ) => {
                {
                    let mut temp_vec = Vec::new();
                    $(
                        temp_vec.push($x);
                    )*
                    temp_vec
                }
            };
        }
    }

    let b = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let a = temp_vec![1; 2];
    println!("a: {:?}", a);
}
