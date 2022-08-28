struct Interface<'a> {
    manager: &'a mut Manager<'a>,
}

impl<'a> Interface<'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a mut String,
}

struct List<'a> {
    manager: Manager<'a>,
}

// impl<'a, 'b> List<'a> {
//     pub fn get_interface(self: &'b mut List<'a>) -> Interface<'b>
//     where
//         'a: 'b,
//     {
//         Interface {
//             manager: &mut self.manager,
//         }
//     }
// }
fn temp<'a, 'b>(s: &'b mut List<'a>) -> &'b mut Manager<'b> {
    &mut s.manager
}
fn temp2<'a, 'b>(s: &'b mut List<'a>) -> &'b mut String {
    s.manager.text
}
fn main() {
    let mut s = String::from("hello");
    let mut list = List {
        manager: Manager { text: &mut s },
    };

    temp(&mut list);

    println!("Interface should be dropped here and the borrow released");

    // 下面的调用会失败，因为同时有不可变/可变借用
    // 但是Interface在之前调用完成后就应该被释放了
    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}
