use std::collections::BTreeMap;

fn main() {
    // Creazione di una nuova BTreeMap con alcune coppie chiave-valore
    let mut map = BTreeMap::new();
    map.insert(1, "uno");
    map.insert(2, "due");
    map.insert(3, "tre");
    map.insert(4, "quattro");
    map.insert(5, "cinque");

    // Utilizzo del metodo range_mut per iterare mutabilmente
    //su un intervallo specificato di chiavi
    let mut range_iter = map.range_mut(2..=4);
    // Itera mutabilmente sulle chiavi da 2 a 4 (inclusi)

    // Modifica dei valori all'interno dell'intervallo specificato
    while let Some((_key, value)) = range_iter.next() {
        *value = "modificato";
    }

    // Stampa della mappa dopo le modifiche
    println!("Mappa dopo le modifiche: {:?}", map);
}
