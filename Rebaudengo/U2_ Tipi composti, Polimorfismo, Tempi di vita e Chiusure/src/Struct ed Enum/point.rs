
use  std::f64;
struct Point {
    x: i32,
    y: i32
}
impl Point {
    fn mirror(self) -> Self {
        Self{ x: self.y, y: self.x }
    }

    fn length(&self) -> f64 {
        let a = self.x*self.x + self.y*self.y;
        let res = (a as f64).sqrt();
        res
    }
    fn scale(&mut self, s: i32) {
        self.x *= s;
        self.y *= s;
    }
}

fn main() {
    let p1 = Point{ x: 3, y: 4 };
    let mut p2 = p1.mirror();
    let l1 = p2.length(); // l1: 5
    println!("{}", l1);
    p2.scale(2);
    let l2 = p2.length(); // l2: 10
    println!("{}", l2);
}



