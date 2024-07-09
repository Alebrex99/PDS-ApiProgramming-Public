fn main() {
    let strings = vec!["hello", "world"];
    let numbers = vec![1, 2, 3];

    // Concateniamo il vettore di stringhe con il vettore di numeri
    let concatenated = strings.iter()
        .map(|s| s.to_string())
        .chain(numbers.iter().map(|&n| n.to_string()));

    for s in concatenated {
        println!("elemento: {:?}", s);
    }

}

