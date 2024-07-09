struct Point {
    x: f32,
    y: f32
}


fn main() {
    let p = Point { x: 5., y: 10. };

    let Point { x: ascissa, y: ordinata} = p;

    println!("The original point was: ({},{})", ascissa, ordinata);
}