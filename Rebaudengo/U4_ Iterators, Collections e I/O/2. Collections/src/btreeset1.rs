use std::collections::BTreeSet;

fn main() {
    let mut set: BTreeSet<i32> = BTreeSet::new();
    // Inserisci elementi nel set
    set.insert(10);
    set.insert(20);
    set.insert(30);

    // Stampa il set per verificare l'inserimento
    println!("{:?}", set);

    // Rimuovi un elemento dal set
    set.remove(&20);

    // Stampa il set per verificare la rimozione
    println!("{:?}", set);

    // Controlla se un elemento è presente nel set
    println!("Is 30 in the set? {}", set.contains(&30));

    // Inizializza un iteratore sul set
    let mut iterator = set.iter();

    // Stampa gli elementi del set attraverso l'iteratore
    println!("Iterating over the set:");
    while let Some(element) = iterator.next() {
        println!("{}", element);
    }
}
