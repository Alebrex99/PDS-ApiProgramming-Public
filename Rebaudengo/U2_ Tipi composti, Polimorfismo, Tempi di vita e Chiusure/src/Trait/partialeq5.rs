#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq<i32> for Point {
    fn eq(&self, other: &i32) -> bool {
        self.x == *other && self.y == *other
    }
}

fn main() {
    let point = Point { x: 5, y: 5 };

    // Confronto tra un punto e un valore i32
    println!("point == 5: {}", point == 5); // true
}