trait HasArea {
    fn get_area(&self) -> f64;
}

struct Circle { radius: f64}
impl HasArea for Circle {
    fn get_area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct Rectangle {    width: f64,    height: f64,}
impl HasArea for Rectangle {
    fn get_area(&self) -> f64
    {
        self.width * self.height
    }
}
fn main() {
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle { width: 4.0, height: 5.0 };
    println!("L'area del cerchio è: {}", circle.get_area());
    println!("L'area del rettangolo è: {}", rectangle.get_area());
}

