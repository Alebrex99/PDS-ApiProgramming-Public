fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Calcola il prodotto di tutti gli elementi utilizzando il metodo fold()
    let product = numbers.iter().fold(1, |acc, n| acc * n);

    println!("Il prodotto di {:?} è {}", numbers, product);
}
