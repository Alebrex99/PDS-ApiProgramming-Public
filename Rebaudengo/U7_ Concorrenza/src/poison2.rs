use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    let data = Arc::new(Mutex::new(0));
    let data_cloned = Arc::clone(&data);
    let _ = thread::spawn(move || {
        let mut num = data_cloned.lock().unwrap();
        *num += 1;
        panic!("Oops! Il thread ha avvelenato il mutex.");
    }).join();
    let result = data.lock(); // Tentiamo di acquisire il mutex nel thread principale
    match result {
        Ok(guard) => { println!("Mutex non avvelenato. Valore: {}", *guard); }
        Err(poisoned) => {
            // Il mutex è avvelenato. Recuperiamo il valore
            let mut guard = poisoned.into_inner();
            println!("Mutex avvelenato. Valore recuperato: {}", *guard);
            // Possiamo modidificare il dato del mutex
            *guard += 1;
            println!("Stato del mutex resettato. Nuovo valore: {}", *guard);
        }
    }
    let result = data.lock(); // Tentiamo di acquisire nuovamente il mutex nel thread principale
    match result {
        Ok(guard) => { println!("Mutex non avvelenato. Valore: {}", *guard); }
        Err(poisoned) => {
            // Il mutex rimane avvelenato, ma possiamo recuperare il valore
            let mut guard = poisoned.into_inner();
            println!("Mutex avvelenato. Valore recuperato: {}", *guard);
            // Possiamo ancora modificare il dato
            *guard += 1;
            println!("Stato del mutex resettato. Nuovo valore: {}", *guard);
        }
    }
}
