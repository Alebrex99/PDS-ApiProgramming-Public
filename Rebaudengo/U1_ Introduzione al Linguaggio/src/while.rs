use std::time::{Duration, Instant};    // Importa dalla libreria standard

fn main() {
    let mut counter = 0;

    let time_limit = Duration::new(1,0); // Crea una durata di 1 secondo

    let start = Instant::now();		  // Determina l’ora attuale

    while (Instant::now() - start) < time_limit {  // Finché non è passato 1 s...
        counter += 1;                              // ...incrementa il contatore
    }
    println!("{}", counter);
}