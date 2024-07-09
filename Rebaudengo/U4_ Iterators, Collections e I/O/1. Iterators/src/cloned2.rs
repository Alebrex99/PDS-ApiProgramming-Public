fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Crea un iteratore che restituisce valori clonati
    let mut iter = numbers.iter().cloned();

    // Filtra e mappa i valori clonati
    let doubled_even_numbers = iter
        .filter(|n| n % 2 == 0)
        .map(|n| n * 2);

    // Stampa i numeri pari raddoppiati
    println!("Doubled even numbers:");
    for n in doubled_even_numbers {
        println!("{}", n);
    }

    // L'iteratore originale non è consumato, quindi possiamo utilizzarlo di nuovo
    println!("Original numbers:");
    for n in numbers {
        println!("{}", n);
    }
}

