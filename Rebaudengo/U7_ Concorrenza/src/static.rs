use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
static COUNTER: AtomicUsize = AtomicUsize::new(0);
fn increment_counter() {
    let new_value = COUNTER.fetch_add(1, Ordering::Relaxed);
    println!("Thread ID: {:?}, New value of COUNTER: {}", std::thread::current().id(), new_value);
}
fn main() {
    let mut handles = vec![];
    // Creare 10 thread e farli eseguire la funzione increment_counter
    for _ in 0..10 {
        let handle = thread::spawn(increment_counter);
        handles.push(handle);
    }
    // Attendere la terminazione dei thread
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final value of COUNTER: {}", COUNTER.load(Ordering::Relaxed)); 
    // Output: Final value of COUNTER: 10
}

