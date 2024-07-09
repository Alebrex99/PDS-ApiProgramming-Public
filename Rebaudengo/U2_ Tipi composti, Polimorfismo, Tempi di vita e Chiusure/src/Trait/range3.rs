fn main() {
    let range = 0..10;

    println!("Does the range 0..10 contain 5? {}", range.contains(&5));  // true
    println!("Does the range 0..10 contain 10? {}", range.contains(&10)); // false

    let inclusive_range = 0..=10;
    println!("Does the inclusive range 0..=10 contain 10? {}", inclusive_range.contains(&10));  // true
}