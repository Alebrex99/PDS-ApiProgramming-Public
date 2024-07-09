use crossbeam::atomic::AtomicCell;
use std::sync::Arc;
use std::thread::JoinHandle;

fn main() {
    let cell = Arc::new(AtomicCell::<i16>::new(0)) ;

    // Crea due thread che incrementano il valore nella cella
    let handles: Vec<JoinHandle<()>> = (0..4).map(|_| {
        let cell = Arc::clone(&cell) ;
        std::thread::spawn(move || {
            for _ in 0..1000 {
                cell.fetch_add(1);
            }
        })
    }).collect();

    // Attendi che entrambi i thread abbiano terminato
    for handle in handles {
        handle.join().unwrap();
    }

    // Stampa il valore finale nella cella
    println!("Final value: {}", cell.load() );
}
