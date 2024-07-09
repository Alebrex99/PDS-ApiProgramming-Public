fn main() {
    let  data = vec![1, 2, 3, 4, 5];

    // Definiamo una closure che può essere chiamata solo una volta
    let  process_data = move || {
        println!("Elaborazione dei dati in corso...");
        let sum: i32 = data.iter().sum();
        println!("La somma dei dati è: {}", sum);
    };

    // Chiamiamo la closure
    process_data();

}

