

use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
static COUNTER: AtomicUsize = AtomicUsize::new(0);
fn update_counter(new_value: usize) {
    let mut expected_value = 0;
    loop {
        // Provare a impostare il valore di counter sul nuovo valore 
match COUNTER.compare_exchange(expected_value, new_value, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => break,
            Err(actual_value) => {
      // Se l'operazione fallisce, aggiorna expected_value con il valore attuale e riprova
                expected_value = actual_value;
            }
        }
    }
}
fn main() {
    let thread = thread::spawn(move || {
        update_counter(1);
    });

    thread.join().unwrap();

    println!("Final value of COUNTER: {}", COUNTER.load(Ordering::Relaxed));
}
