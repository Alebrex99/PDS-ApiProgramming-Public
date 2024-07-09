use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
fn main() {
    const NUM_THREADS: usize = 4;
    const NUM_INCREMENTS: usize = 100000;

    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    for _ in 0..NUM_THREADS {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..NUM_INCREMENTS {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final counter value: {}", counter.load(Ordering::SeqCst));
}
