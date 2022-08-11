use std::ops::Add;
use std::ops::AddAssign;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
impl<T: AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let mut p = Point { x: 2, y: 2 };
    let p1 = Point { x: 3, y: 2 };
    p += p1;
    println!("{:?}", p)
}