#[derive(Debug, Eq, PartialEq, PartialOrd)]

struct Point {
    x: i32,
    y: i32,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.x != other.x {
            self.x.cmp(&other.x)
        } else {
            self.y.cmp(&other.y)
        }
    }
}

fn main() {
    let point1 = Point { x: 3, y: 5 };
    let point2 = Point { x: 3, y: 8 };

    // Trova il punto con le coordinate massime usando il metodo max
    let max_point = point1.max(point2);

    println!("Il punto con coordinate massime è {:?}", max_point);
}