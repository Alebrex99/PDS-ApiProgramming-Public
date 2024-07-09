fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Find the index of the first even number
    let index = numbers.iter().position(|&x| x % 2 == 0);

    match index {
        Some(i) => println!("The first even number is at index {}", i),
        None => println!("No even number found"),
    }
}
