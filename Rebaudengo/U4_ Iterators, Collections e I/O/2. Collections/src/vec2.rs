fn main() {
    // Creiamo un nuovo vettore
    //con una capacità iniziale
    let mut vec = Vec::with_capacity(4);

    // Aggiungiamo elementi al vettore
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

    // Stampiamo la capacità del vettore
    println!("Capacità:{}", vec.capacity());

    // Rimuoviamo l'ultimo elemento dal vettore
    let popped_element = vec.pop();
    println!("Rimosso: {:?}", popped_element);

    // Inseriamo un nuovo elemento al terzo indice
    vec.insert(2, 6);

    // Rimuoviamo l'elemento al secondo indice
    let removed_element = vec.remove(1);
    println!("Rimosso: {:?}", removed_element);

    // Accediamo al primo e all'ultimo elemento
    if let Some(first_element) = vec.first() {
        println!("Primo elemento: {}", first_element);
    }
    if let Some(last_element) = vec.last() {
        println!("Ultimo: {}", last_element);
    }

// Accediamo ai primi due elementi del vettore in // modo mutabile
    if let Some(first_mut) = vec.first_mut() {
        *first_mut = 10;
    }

    if let Some(second_mut) = vec.get_mut(1) {
        *second_mut = 20;
    }

// Accediamo ai primi tre elementi del vettore
    println!("Primi 3: {:?}", vec.get(..3).unwrap());

// Accediamo ai primi tre elementi del vettore in // modo mutabile
    if let Some(slice) = vec.get_mut(..3) {
        for elem in slice {
            *elem *= 2;
        }
    }

// Stampiamo il vettore modificato
    println!("Vettore modificato: {:?}", vec);
}