enum Shape {
    Square { s: f64 },
    Circle { r: f64 },
    Rectangle { w: f64, h: f64 }
}

fn compute_area(shape: Shape) -> f64 {
    match shape {
        Shape::Square { s } => s*s,
        Shape::Circle  { r } => r*r*3.1415926,
        Shape::Rectangle {w, h} => w*h,
    }
}

fn main() {
    // initialize enum with values
    let quadrato = Shape::Square{s: 1.0};
    let cerchio = Shape::Circle{r: 2.0};
    let rettangolo = Shape::Rectangle{w: 2.0, h: 3.0};

    println!("Area del Quadrato {}", compute_area(quadrato));
    println!("Area del Cerchio {}", compute_area(cerchio));
    println!("Area del Rettangolo {}", compute_area(rettangolo));
}