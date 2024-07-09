fn main() {
    let multiply = |x: i32, y: i32| -> i64 {
        (x * y).into() };
    println!("{}", multiply(3, 4)); // prints 12
}