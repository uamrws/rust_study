mod macros {
    // #[macro_export]
    // macro_rules! unit {
    //     ($x:expr) => {
    //         ()
    //     };
    // }
    // #[macro_export]
    // macro_rules! count {
    //     ($($x:expr),*) => {
    //         (&[$(unit!($x)),*]).len()
    //     };
    // }
    // #[macro_export]
    // macro_rules! hashmap {
    //     ($($key:expr => $value:expr),* $(,)?) => {
    //         {
    //             let mut temp_map = std::collections::HashMap::with_capacity(count!($($key),*));
    //             $(
    //                 temp_map.insert($key, $value);
    //             )*
    //             temp_map
    //         }
    //     };
    // }
    pub fn new_hashmap<K, V>(num: usize) -> std::collections::HashMap<K, V> {
        std::collections::HashMap::with_capacity(num)
    }
    #[macro_export]
    macro_rules! hashmap {
        (@unit $x:expr) => (());
        (@count $($x:expr),*) =>([$(hashmap!(@unit $x)),*].len());
        ($($key:expr => $value:expr),* $(,)?) => {
            {
                let mut temp_map = $crate::new_hashmap(hashmap!(@count $($key),*));
                $(
                    temp_map.insert($key, $value);
                )*
                temp_map
            }
        };
    }
}
use macros::new_hashmap;
fn main() {
    let b = "2:";
    let a = hashmap! {
        "1:"=> 1,
        b=> 2,
        "3:"=> 3,
    };
    assert_eq!(a.capacity(), 3);
    assert_eq!(a["1:"], 1);
}
