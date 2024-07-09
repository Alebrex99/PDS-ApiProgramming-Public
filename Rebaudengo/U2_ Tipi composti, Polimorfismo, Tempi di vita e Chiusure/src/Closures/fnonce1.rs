fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    // Definiamo una chiusura che prende il possesso del vettore vec
    let consume_vector = move || {
        // Consumiamo il vettore stampando tutti i suoi elementi
        for num in vec {
            println!("{}", num);
        }
    };

    // Chiamiamo la chiusura. Dopo questa chiamata, non possiamo più usarla.
    consume_vector();

    // Tentare di chiamare nuovamente la chiusura genererebbe un errore di compilazione.
    // consume_vector();
}
