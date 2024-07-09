#[derive(Debug)]
enum Shape {
    Square { s: f64 },
    Circle { r: f64 },
    Rectangle { w: f64, h: f64 }
}

fn process (shape: & mut Shape) {
    if let Shape::Square {ref mut s} = shape {
        *s *= 2.0;
    }

}