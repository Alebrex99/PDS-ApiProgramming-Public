use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.insert(1, "uno");
    map.insert(2, "due");
    map.insert(3, "tre");
    map.insert(4, "quattro");
    map.insert(5, "cinque");
    map.insert(6, "sei");
    map.insert(7, "sette");
    map.insert(8, "otto");
    map.insert(9, "nove");
    map.insert(10, "dieci");

    // Utilizzo del metodo range per iterare su un intervallo specificato di chiavi
    let mut range_iter = map.range(3..8); // Itera sulle chiavi da 3 a 7 (inclusi)

    // Stampa degli elementi nell'intervallo specificato
    println!("Elementi nell'intervallo da 3 a 7:");
    while let Some((key, value)) = range_iter.next() {
        println!("Chiave: {}, Valore: {}", key, value);
    }
}