use std::process;

fn main() {
    println!("Esempio di programma in Rust che usa exit()");

    // Condizione per terminare il programma
    let some_condition = true;

    if some_condition {
        println!("Condizione soddisfatta, il programma terminerà con codice 1.");
        process::exit(1);
    }
    // Se il programma non è terminato prima, proseguirà con altre operazioni

    // Terminare il programma con codice 0 (successo)
    process::exit(0);
}

