fn main() {
    let numbers = vec![1, 2, 3, 4];
    let new_numbers = numbers.iter()
        .flat_map(|&x| vec![x, x * x, x * x * x]);

    for n in new_numbers {
        println!("{:?}", n);
    } // Output: [1, 1, 1, 2, 4, 8, 3, 9, 27, 4, 16, 64]
}

