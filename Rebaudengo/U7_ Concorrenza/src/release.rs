use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
const BUFFER_SIZE: usize = 10;
fn main() {
    let buffer = Arc::new(Mutex::new(Vec::with_capacity(BUFFER_SIZE)));
    let producer_finished = Arc::new(AtomicBool::new(false));
    let buffer_clone = Arc::clone(&buffer);
    let producer_finished_clone = Arc::clone(&producer_finished);
    let producer_handle = thread::spawn(move || {
        for i in 0..10 {
            thread::sleep(std::time::Duration::from_nanos(1));
            let mut buffer = buffer_clone.lock().unwrap();
            while buffer.len() >= BUFFER_SIZE {
		buffer = buffer_clone.lock().unwrap(); // Attendi finché il buffer è pieno
            }
            buffer.push(i); // Aggiungi un valore al buffer
            println!("Produttore: prodotto {}", i);
        }
        producer_finished_clone.store(true, Ordering::Release);
    });
    let buffer_clone = Arc::clone(&buffer);
    let consumer_handle = thread::spawn(move || {
        loop {
            let mut buffer = buffer_clone.lock().unwrap();
            let len = buffer.len();
            if len > 0 {
                let value = buffer.remove(0); // Se ci sono elementi nel buffer, consumane uno
                println!("Consumatore: consumato {}", value);
            } else if producer_finished.load(Ordering::Acquire) {
	break; // Se il produttore ha finito, esce dal ciclo
            }
            // Altrimenti, rilascia il lock e attendi finché ci sono elementi nel buffer o il produttore ha finito
        }
        println!("Tutti gli elementi sono stati consumati");
    });
    producer_handle.join().unwrap();
    consumer_handle.join().unwrap();
}

