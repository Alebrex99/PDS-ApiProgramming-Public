fn main() {
    let numbers = vec![1, 3, 5];

    // Find the index of the first even number
    let index = numbers.iter().rposition(|&x| x % 2 == 0);

    match index {
        Some(i) => println!("The last even number is at index {}", i),
        None => println!("No even number found"),
    }
}

