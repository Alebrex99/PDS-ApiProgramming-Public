#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.x < other.x && self.y < other.y {
            Some(std::cmp::Ordering::Less)
        } else if self.x > other.x && self.y > other.y {
            Some(std::cmp::Ordering::Greater)
        } else if self.x == other.x && self.y == other.y {
            Some(std::cmp::Ordering::Equal)
        } else {
            None // In caso di confronto non definito
        }
    }
}

fn main() {
    let point1 = Point { x: 1.0, y: 4.0 };
    let point2 = Point { x: 3.0, y: 2.0 };


    match point1.partial_cmp(&point2) {
        Some(ordering) => {
            println!("point1 è {:?} di point2", ordering);
        }
        None => {
            println!("Confronto non definito tra point1 e point2");
        }
    }

}