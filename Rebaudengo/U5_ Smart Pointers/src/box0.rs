fn main() {
    // Allocazione di un intero sulla heap con Box.
    let mut numero = Box::new(10);

    println!("Valore iniziale: {}", numero);

    // Modifica del valore all'interno del Box.
    *numero = 20;

    println!("Valore modificato: {}", numero);
}
