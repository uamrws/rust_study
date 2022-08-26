struct Interface<'a, 'b> {
    manager: &'a mut Manager<'b>,
}

impl<'a, 'b> Interface<'a, 'b> {
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

impl<'a, 'b> List<'a> {
    pub fn get_interface(self: &'b mut List<'a>) -> Interface<'b, 'a> {
        Interface {
            manager: &mut self.manager,
        }
    }
}

fn main() {
    let mut s = String::from("hello");
    let mut list = List {
        manager: Manager { text: &mut s },
    };

    let mut x = &mut list;
    println!("{:p}", x.manager.text);
    {
        let mut ss = String::from("hhhh");
        let xx = &mut ss;
        x.manager.text = xx;
        println!("{:p}", x.manager.text);

        x.get_interface();
        use_list(&list);
    }
}

fn use_list(list: &List) {}
