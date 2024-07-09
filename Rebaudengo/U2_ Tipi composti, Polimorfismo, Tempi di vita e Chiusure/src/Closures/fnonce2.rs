fn main() {
    let mut data = vec![1, 2, 3, 4, 5];

    // Definiamo una closure che può essere chiamata solo una volta
    let mut process_data = move || {
        println!("Elaborazione dei dati in corso...");
        let sum: i32 = data.iter().sum();
        println!("La somma dei dati è: {}", sum);
        drop(data);
        // Consumiamo i dati per evitare ulteriori elaborazioni
    };

    // Chiamiamo la closure
    process_data();

    // Tentiamo di chiamare nuovamente la closure
    // Questo produrrà un errore di compilazione perché la closure è stata consumata
    //process_data();
}
