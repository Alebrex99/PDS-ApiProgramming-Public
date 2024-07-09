use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    // Creiamo una struttura dati condivisa protetta da un RwLock
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));

    // Creiamo 10 thread che leggono dalla struttura dati condivisa
    let mut threads = vec![];

    for i in 0..10 {
        // Cloniamo l'arc per ogni thread in modo che ciascuno abbia un riferimento condiviso
        let data_clone = Arc::clone(&data);

        // Creiamo il thread
        let thread = thread::spawn(move || {
            // Otteniamo un blocco di lettura (non esclusivo) sul RwLock
            let guard = data_clone.read().unwrap();
            println!("Thread {}: {:?}", i, *guard);
        });

        threads.push(thread);
    }

    // Attendo che tutti i thread terminino
    for thread in threads {
        thread.join().unwrap();
    }
}
