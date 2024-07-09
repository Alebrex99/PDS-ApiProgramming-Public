use std::sync::{Arc, Mutex, Condvar};
use std::thread;
fn main() {
    const NUM_THREADS: usize = 5;
    let data = Arc::new((Mutex::new(0), Condvar::new()));
    let mut handles = vec![];
    for i in 0..NUM_THREADS {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let (lock, cvar) = &*data_clone;
            let mut data_guard = lock.lock().unwrap();
            println!("Thread {} in attesa...", i);
            // Attendi fino a quando il dato non è diverso da 0
            while *data_guard == 0 {
                data_guard = cvar.wait(data_guard).unwrap();
            }
            println!("Thread {} svegliato! Il dato è: {}", i, *data_guard);
        });
        handles.push(handle);
    }
    // Facciamo avanzare i thread dopo 2 secondi
    thread::sleep(std::time::Duration::from_secs(2));
    // Cambiamo il dato condiviso e svegliamo tutti i thread
    {
        let (lock, cvar) = &*data;
        let mut data_guard = lock.lock().unwrap();
        *data_guard = 42;
        cvar.notify_all();
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
