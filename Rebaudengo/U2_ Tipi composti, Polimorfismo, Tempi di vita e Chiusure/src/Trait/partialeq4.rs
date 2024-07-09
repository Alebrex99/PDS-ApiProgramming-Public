
#[derive(PartialEq)]

struct FloatingPointNumber {
    value: f64,
}

fn main() {
    let a = FloatingPointNumber { value: 1.0 };
    let b = FloatingPointNumber { value: 1.01 };
    let c = FloatingPointNumber { value: 1.000000001 };
    let d = FloatingPointNumber { value: 1.000000001 };

    println!("a == b: {}", a == b);
    println!("a == c: {}", a == c);
    println!("c == d: {}", c == d);
}