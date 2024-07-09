use std::collections::HashSet;

fn main() {
    let mut numbers_set: HashSet<i32> = HashSet::new();
    // Inseriamo alcuni numeri nel set
    numbers_set.insert(1);
    numbers_set.insert(2);
    numbers_set.insert(3);
    numbers_set.insert(4);

    // Controlliamo se il set contiene un certo numero
    println!("Il set contiene il numero 3? {}", numbers_set.contains(&3));
    // Stampa: Il set contiene il numero 3? true
    println!("Il set contiene il numero 5? {}", numbers_set.contains(&5));
    // Stampa: Il set contiene il numero 5? false

    println!("Numero di elementi nel set: {}", numbers_set.len());
    // Stampa: Numero di elementi nel set: 4

    println!("Il set è vuoto? {}", numbers_set.is_empty()); // Stampa: Il set è vuoto? false

    numbers_set.remove(&4); // Rimuoviamo un numero dal set

    // Iteriamo attraverso gli elementi del set e stampiamoli
    println!("Elementi nel set:");
    for number in &numbers_set {
        println!("{}", number);
    } // Stampa: 1, 2, 3

    // Rimuoviamo tutti gli elementi dal set
    numbers_set.clear();
    println!("Numero di elementi nel set dopo la cancellazione: {}", numbers_set.len());
}