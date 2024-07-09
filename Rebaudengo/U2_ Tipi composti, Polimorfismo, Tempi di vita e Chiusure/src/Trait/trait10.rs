trait Shape {
    fn area(&self) -> f64;
}

trait Drawable: Shape {
    fn draw(&self);
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle with width {} and height {}.", self.width, self.height);
    }
}

struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square with side {}.", self.side);
    }
}
fn draw_shape(shape: &dyn Drawable) {
    shape.draw();
    println!("Area: {}", shape.area());
}
fn main() {
    let rectangle = Rectangle { width: 3.0, height: 4.0 };
    let square = Square { side: 2.0 };

    draw_shape(&rectangle);
    draw_shape(&square);
}
