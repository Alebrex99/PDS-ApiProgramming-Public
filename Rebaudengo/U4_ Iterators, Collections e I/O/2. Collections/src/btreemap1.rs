use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.insert(3, "tre");
    map.insert(1, "uno");
    map.insert(4, "quattro");
    map.insert(2, "due");
    map.insert(5, "cinque");

    println!("Mappa: {:?}", map);

    // Verifica se una chiave è presente nella mappa
    println!("La chiave 2 è presente nella mappa: {}", map.contains_key(&2));

    // Accesso all'elemento associato a una chiave
    if let Some(value) = map.get(&3) {
        println!("Valore associato alla chiave 3: {}", value);
    }

    // Rimozione di un elemento dalla mappa
    let removed_value = map.remove(&4);
    match removed_value {
        Some(value) => println!("Elemento rimosso: {}", value),
        None => println!("La chiave non esisteva nella mappa"),
    }
    // Iterazione sugli elementi della mappa
    println!("Iterazione sulla mappa:");
    for (key, value) in &map {
        println!("Chiave: {}, Valore: {}", key, value);
    }
}