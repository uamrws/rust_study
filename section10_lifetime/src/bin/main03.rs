// #[derive(Debug)]
// struct Foo;

// impl Foo {
//     fn mutate_and_share(&mut self) -> &Self {
//         &*self
//     }
//     fn share(&self) {}
// }

// fn main() {
//     let mut foo = Foo;
//     let loan = foo.mutate_and_share();
//     foo.share();
//     println!("{:?}", loan);
// }

#![allow(unused)]
fn main() {
    use std::collections::HashMap;
    use std::hash::Hash;
    fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
    where
        K: Clone + Eq + Hash,
        V: Default,
    {
        if let Some(v) = map.get_mut(&key) {
            return v;
        }
        map.insert(key.clone(), V::default());
        map.get_mut(&key).unwrap()
    }
}
