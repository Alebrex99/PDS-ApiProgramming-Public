fn main() {
    let numbers = vec![1, 2, 3];

    // Calcola il prodotto di tutti gli elementi dell'iteratore
    let product: u32 = numbers.iter().product();

    println!("Il prodotto di {:?} è {}", numbers, product);
}
