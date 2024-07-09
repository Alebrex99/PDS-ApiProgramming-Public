use std::collections::BinaryHeap;

fn main() {
    // Creazione di una BinaryHeap con alcuni valori non ordinati
    let mut heap = BinaryHeap::from(vec![10, 5, 8, 3, 12]);

    // Chiamata del metodo into_sorted_vec per ottenere un vettore ordinato
    let sorted_vec = heap.into_sorted_vec();

    // Stampa del vettore ordinato
    println!("Vettore ordinato: {:?}", sorted_vec);
}