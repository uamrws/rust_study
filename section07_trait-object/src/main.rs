// trait Draw {
//     fn draw(&self);
// }
// struct Button {
//     width: u32,
//     height: u32,
//     label: String,
// }

// impl Draw for Button {
//     fn draw(&self) {
//         // 绘制按钮的代码
//     }
// }

// struct SelectBox {
//     width: u32,
//     height: u32,
//     options: Vec<String>,
// }

// impl Draw for SelectBox {
//     fn draw(&self) {
//         // 绘制SelectBox的代码
//     }
// }
// struct Screen {
//     components: Vec<Box<dyn Draw>>,
// }

// impl Screen {
//     fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }
// fn main() {
//     let screen = Screen {
//         components: vec![
//             Box::new(SelectBox {
//                 width: 75,
//                 height: 10,
//                 options: vec![
//                     String::from("Yes"),
//                     String::from("Maybe"),
//                     String::from("No"),
//                 ],
//             }),
//             Box::new(Button {
//                 width: 50,
//                 height: 10,
//                 label: String::from("OK"),
//             }),
//         ],
//     };

//     screen.run();
// }

trait Temp {
    fn temp(self);
}
#[derive(Debug)]
struct Temp2 {}
impl Temp for Temp2 {
    fn temp(self: Self) {}
}
fn main() {
    let t = Temp2 {};
    t.temp();
    println!("{:?}", t)
}
