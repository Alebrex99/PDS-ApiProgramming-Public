use std::sync::Arc;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);
    let handle = thread::spawn(move || {
        while running_clone.load(Ordering::Relaxed) {
            println!("Working...");
            thread::sleep(std::time::Duration::from_secs(1));
        }
        println!("Thread exiting...");
    });
    // Simulate main thread doing some work
    thread::sleep(std::time::Duration::from_secs(5));

    // Set the running flag to false to signal the thread to exit
    running.store(false, Ordering::Relaxed);
    
    // Wait for the spawned thread to finish
    handle.join().unwrap();
}
