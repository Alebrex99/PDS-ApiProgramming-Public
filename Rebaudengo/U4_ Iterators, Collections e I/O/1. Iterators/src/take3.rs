fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    let first_two_over_twenty = numbers.iter()
        .take(3)
        .filter(|&num| *num > 10);

    for n in first_two_over_twenty {
    println!("Numeri maggiori di 10 tra i primi 3: {:?}", n);
    }
}
