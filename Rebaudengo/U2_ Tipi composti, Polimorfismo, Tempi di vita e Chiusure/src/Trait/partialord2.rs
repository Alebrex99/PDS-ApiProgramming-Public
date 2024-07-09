#[derive(Debug, PartialEq, PartialOrd)]
struct Point {
    x: f64,
    y: f64,
}


fn main() {
    let point1 = Point { x: 1.0, y: 2.0 };
    let point2 = Point { x: 3.0, y: 4.0 };

    println!("point1 < point2: {}", point1.lt(&point2));
    println!("point1 > point2: {}", point1.gt(&point2));
}