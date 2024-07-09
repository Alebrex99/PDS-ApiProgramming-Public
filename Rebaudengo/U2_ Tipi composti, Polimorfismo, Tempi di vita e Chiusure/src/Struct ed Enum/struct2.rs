struct Point (i32, i32, i32);

fn main() {
    let p1 = Point(0, 0, 0);
    let p2 = (0, 0, 0);
    println!("{:?} \n{:?}", p1, p2);
    println!("{} {} {}", p1.0, p1.1, p1.2);
}