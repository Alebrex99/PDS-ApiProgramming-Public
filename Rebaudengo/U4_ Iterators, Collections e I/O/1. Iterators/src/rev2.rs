fn main() {
        // Creazione di un range da 1 a 5
        let range = 1..=5;

        // Inversione dell'ordine del range utilizzando il metodo rev
        let reversed_range= range.rev();

        for n in reversed_range {
                println!("Nuovo Range: {:?}", n);
        }
}

