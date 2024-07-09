struct FloatingPointNumber {
    value: f64,
}

impl PartialEq for FloatingPointNumber {
    fn eq(&self, other: &Self) -> bool {
        let epsilon = 0.00001;
        let difference = (self.value - other.value).abs();
        difference <= epsilon
    }
}

fn main() {
    let a = FloatingPointNumber { value: 1.0 };
    let b = FloatingPointNumber { value: 1.01 };
    let c = FloatingPointNumber { value: 1.000000001 };

    println!("a == b: {}", a == b);
    println!("a == c: {}", a == c);
}
