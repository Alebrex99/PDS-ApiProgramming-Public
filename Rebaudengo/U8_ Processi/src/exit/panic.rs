fn main() {
    // Esegui alcune operazioni
    println!("Esempio di programma in Rust che usa panic!");

    // Condizione per far scattare il panic
    let some_condition = true;

    if some_condition {
        println!("Condizione soddisfatta, il programma terminerà con un panic.");
        panic!("Errore: si è verificato un panic!");
    }

    // Se il programma non è terminato prima, proseguirà con altre operazioni
    println!("Il programma continuerà a funzionare normalmente.");
}
