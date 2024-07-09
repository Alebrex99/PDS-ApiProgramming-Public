fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    // Prende i primi due numeri maggiori di 20 dall'inizio del vettore
    let first_two_over_twenty = numbers.iter()
        .filter(|&num| *num > 20)
        .take(2);
    for n in first_two_over_twenty {
    println!("Primi due numeri maggiori di 20: {:?}", n);
    }
}
