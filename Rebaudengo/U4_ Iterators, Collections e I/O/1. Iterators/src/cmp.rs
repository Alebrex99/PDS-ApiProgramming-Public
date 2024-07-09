fn main() {
    let numbers1 = vec![1, 3, 2];
    let numbers2 = vec![1, 2, 4];

    // Confronto dei due vettori
    let comparison = numbers1.iter().cmp(numbers2.iter());

    match comparison {
        std::cmp::Ordering::Less => println!("Il primo vettore è minore"),
        std::cmp::Ordering::Equal => println!("I due vettori sono uguali"),
        std::cmp::Ordering::Greater => println!("Il primo vettore è maggiore"),
    }
}
