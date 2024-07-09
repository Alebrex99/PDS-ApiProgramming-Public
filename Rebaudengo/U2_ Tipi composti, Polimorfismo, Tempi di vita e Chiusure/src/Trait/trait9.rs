trait Shape { fn area(&self) -> f64; }

struct Circle { radius: f64}
impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}

struct Rectangle {
    width: f64,
    height: f64
}
impl Shape for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 3.0, height: 4.0 };

    let mut shapes: Vec<&dyn Shape> = Vec::new();
    shapes.push(&circle);
    shapes.push(&rectangle);
    for shape in shapes {
        println!("Area: {}", shape.area());
    }
}
