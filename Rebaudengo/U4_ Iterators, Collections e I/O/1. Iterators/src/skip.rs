fn main() {
    let numbers = vec![1, 4, 5, 6, 7];

    // Salta i primi due numeri
    let skipped = numbers.iter().skip(2);

    for n in skipped {
    println!("Valori dopo i primi 2: {:?}", n);
    }
}
