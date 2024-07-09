use std::collections::HashSet;

fn main() {
    let mut numeri = HashSet::new();
    numeri.insert(1);
    numeri.insert(2);
    numeri.insert(3);

    // Usare get per verificare se un elemento è presente
    if numeri.get(&2).is_some() {
        println!("Il numero 2 è presente nel HashSet.");
    }

    // Usare take per rimuovere e restituire un elemento
    if let Some(numero) = numeri.take(&3) {
        println!("Il numero {} è stato rimosso dal HashSet.", numero);
    } else {
        println!("Il numero 3 non era presente"); }

    // Verificare che il numero 3 sia stato rimosso
    if numeri.get(&3).is_none() {
        println!("Il numero 3 non è più presente nel HashSet.");
    }

    let vecchio_numero = 2;
    let nuovo_numero = 4;
    // Rimuovere il vecchio numero se presente e inserire il nuovo
    if numeri.remove(&vecchio_numero) {
        numeri.insert(nuovo_numero);
        println!("Il numero {} è stato sostituito con {} nel HashSet.", vecchio_numero, nuovo_numero);
    }
    println!("Contenuto finale del HashSet: {:?}", numeri);
}
