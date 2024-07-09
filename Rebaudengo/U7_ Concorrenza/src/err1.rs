
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

fn main() {
    // Create an Arc (atomic reference counting) pair containing a Mutex and a Condvar
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    // Inside our lock, spawn a new thread and wait for it to start
    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true; // We notify the Condvar that the value has changed

        cvar.notify_one();
    });

    // Wait for the thread to start up
    let (lock, cvar) = &*pair;


    println!("Waiting ...");
    thread::sleep(Duration::from_secs(1));
    let mut started = lock.lock().unwrap();
    started = cvar.wait(started).unwrap();
    println!("Thread started!");
}
