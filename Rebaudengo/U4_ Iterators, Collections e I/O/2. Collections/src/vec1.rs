fn main() {
    // Creiamo un nuovo vettore vuoto
    let mut vec = Vec::new();

    // Verifichiamo se il vettore è vuoto
    println!("Il vettore è vuoto? {}", vec.is_empty());

    // Aggiungiamo alcuni elementi al vettore
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Stampiamo la lunghezza del vettore dopo l'aggiunta degli elementi
    println!("Nuova lunghezza del vettore: {}", vec.len());

    // Creiamo un iteratore dal vettore
    let iter = vec.iter();

    // Iteriamo sul vettore utilizzando l'iteratore
    println!("Elementi del vettore:");
    for num in iter {
        println!("{}", num);
    }

    // Convertiamo il vettore in un iteratore e raccogliamo i risultati in un nuovo vettore
    let new_vec: Vec<_> = vec.into_iter().collect();

    // Stampiamo il nuovo vettore
    println!("{:?}", new_vec);
}
