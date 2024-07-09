struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    fn with_square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    let rectangle = Rectangle::new(5, 10);
    let area = rectangle.calculate_area();
    println!("Area: {}", area);
    let square = Rectangle::with_square(10);
    let area = square.calculate_area();
    println!("Area: {}", area);
}