use std::sync::Arc;
#[derive(Clone)]
struct Container<T>(Arc<T>);

fn clone_containers<T>(foo: &Container<i32>, bar: &Container<T>) {
    let foo_cloned = foo.clone();
    let bar_cloned = bar.clone();
    println!("{}", bar_cloned as *const Container<T> as usize)
}

fn main() {
    let a = &Container(Arc::new(12));
    let b = &Container(Arc::new(12));
    clone_containers(a, b)
}
