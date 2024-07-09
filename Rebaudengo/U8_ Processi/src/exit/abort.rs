
use std::process;

fn main() {
    // Esegui alcune operazioni
    println!("Esempio di programma in Rust che usa abort()");

    // Condizione per terminare il programma con un abort
    let some_condition = true;

    if some_condition {
        println!("Condizione soddisfatta, il programma terminerà con abort.");
        process::abort();
    }

    // Se il programma non è terminato prima, proseguirà con altre operazioni
    println!("Il programma continuerà a funzionare normalmente.");
}
