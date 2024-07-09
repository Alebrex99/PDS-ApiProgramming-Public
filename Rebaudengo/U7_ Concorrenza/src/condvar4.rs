use std::{
    sync::{Arc, Condvar, Mutex},
    thread::{sleep},
    time::Duration,
};
struct Counter {
    value: Mutex<u32>,
    condvar: Condvar,
}
fn main() {
    let counter = Arc::new(Counter {
        value: Mutex::new(0),
        condvar: Condvar::new(),
    });
    let counter_clone = counter.clone();
    let  counting_thread =   std::thread::spawn(move || loop {
        sleep(Duration::from_millis(100));
        let mut value = counter_clone.value.lock().unwrap();
        *value += 1;
        counter_clone.condvar.notify_all();
        if *value > 15 {
            break;
        }
    });
    // Wait until the value more or equal to 15
    let mut value = counter.value.lock().unwrap();
    value = counter.condvar.wait_while(value, |val| *val < 15).unwrap();
    println!("Condition met. Value is now {}.", *value);
    drop(value); // Unlock value
    // Wait for counting thread to finish
    counting_thread.join().unwrap();
}

