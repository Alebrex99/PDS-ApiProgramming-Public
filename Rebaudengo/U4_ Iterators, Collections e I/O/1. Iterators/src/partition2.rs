fn main() {
    let people = vec![
        ("Alice", 30),
        ("Bob", 25),
        ("Charlie", 35),
        ("David", 40),
    ];

    // Dividi la collezione in due parti in base all'età
    let (young, old): (Vec<_>,Vec<_>) = people.into_iter().partition(|&(_, age)| age < 35);

    println!("Giovani: {:?}", young);
    println!("Anziani: {:?}", old);
}
