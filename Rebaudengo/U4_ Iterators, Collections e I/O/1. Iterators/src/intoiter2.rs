fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Consuma il vettore e calcola la somma dei suoi elementi utilizzando into_iter()
    let sum: i32 = numbers.into_iter().sum();

    // Questo non è consentito, poiché numbers è già stato consumato
    // println!("{:?}", numbers);

    // Stampa la somma dei numeri
    println!("Somma: {}", sum); // Output: Somma: 15
}