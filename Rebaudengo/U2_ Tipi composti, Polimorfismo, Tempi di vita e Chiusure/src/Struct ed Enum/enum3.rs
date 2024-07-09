enum Shape {
    Square { s: f64 },
    Circle { r: f64 },
    Rectangle { w: f64, h: f64 }
}

fn process (shape: Shape) -> () {
    if let Shape::Square {s} = shape {
        println!("E' un quadrato")
    }
    else {
        println!("Non è un quadrato")
    }
}

fn main() {
    let cerchio = Shape::Circle{r: 2.0};
    process(cerchio);

}

