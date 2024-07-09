fn main() {
    let numbers = vec![1, 40, 5, 60, 7, 45, 34, 99];

    // Salta i primi due numeri
    let skipped = numbers.iter()
        .skip(2)
        .filter(|&num| *num > 10)
        .take(3);
    ;

    for n in skipped {
    println!("Dopo i primi 2, i primi 3 valori superiori a 10: {:?}", n);
    }
}
