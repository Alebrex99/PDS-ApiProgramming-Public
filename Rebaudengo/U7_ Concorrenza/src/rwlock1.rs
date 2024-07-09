use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    // Creiamo una struttura dati condivisa protetta da un RwLock
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    // Cloniamo l'arc per ogni thread in modo che ciascuno abbia un riferimento condiviso
    let data_clone1 = Arc::clone(&data);
    let data_clone2 = Arc::clone(&data);

    // Thread che legge dalla struttura dati condivisa
    let reader = thread::spawn(move || {
        // Otteniamo un blocco di lettura (non esclusivo) sul RwLock
        let guard = data_clone1.read().unwrap();
        println!("Thread lettore: {:?}", guard);
    });

    // Thread che scrive sulla struttura dati condivisa
    let writer = thread::spawn(move || {
        // Otteniamo un blocco di scrittura (esclusivo) sul RwLock
        let mut guard = data_clone2.write().unwrap();
        guard.push(4);
        println!("Thread scrittore: {:?}", guard);
    });

    // Attendo che entrambi i thread terminino
    reader.join().unwrap();
    writer.join().unwrap();
}
