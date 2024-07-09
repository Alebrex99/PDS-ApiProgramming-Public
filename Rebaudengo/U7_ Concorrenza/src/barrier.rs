use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    // Numero di thread da sincronizzare
    let num_threads = 10;

    // Crea una barriera per sincronizzare num_threads thread
    let barrier = Arc::new(Barrier::new(num_threads));

    let mut handles = Vec::with_capacity(num_threads);

    for i in 0..num_threads {
        let barrier_clone = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("Thread {} sta facendo un po' di lavoro", i);

            // Simula il lavoro con una sleep crescente per thread
            thread::sleep(std::time::Duration::from_secs((i+1).try_into().unwrap()));

            println!("Thread {} è arrivato alla barriera", i);

            // Attende che tutti i thread raggiungano la barriera
            barrier_clone.wait();

            println!("Thread {} ha superato la barriera", i);
        });

        handles.push(handle);
    }

    // Attende che tutti i thread completino il loro lavoro
    for handle in handles {
        handle.join().unwrap();
    }
}
