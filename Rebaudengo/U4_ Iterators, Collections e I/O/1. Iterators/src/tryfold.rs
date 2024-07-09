fn main() {
    let numbers = vec![2, 3, 0, 5];

    // Calcola il prodotto di tutti gli elementi della collezione
    let product = numbers.iter().try_fold(1, |acc, &x| {
        if x != 0 {
            Ok(acc * x)
        } else {
            Err("Zero")
        }
    });

    match product {
        Ok(result) => println!("Il prodotto di tutti gli elementi è: {}", result),
        Err(err) => println!("Errore: {}", err),
    }
}