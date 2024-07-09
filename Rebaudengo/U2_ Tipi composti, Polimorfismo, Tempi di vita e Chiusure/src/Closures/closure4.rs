fn main() {
    let mut numbers = vec![5, 2, 9, 1, 7];

    // Sorting the vector in ascending order
    numbers.sort_by(|a, b| a.cmp(b));

    println!("Sorted numbers: {:?}", numbers);
}