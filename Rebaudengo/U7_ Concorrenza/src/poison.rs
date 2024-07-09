use std::time::Duration;
use std::sync::{Arc, RwLock};
use std::thread;
fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let data_clone1 = Arc::clone(&data);
    let data_clone2 = Arc::clone(&data);
    let data_clone3 = Arc::clone(&data);
    let reader1 = thread::spawn(move || {// Thread in lettura 1
        let guard = data_clone1.read().unwrap();
        println!("Thread lettura 1: {:?}", *guard);
    });
    let reader2 = thread::spawn(move || {// Thread in lettura 2
        thread::sleep(Duration::from_secs(1));
        let guard = data_clone2.read();
        match guard {
            Ok(guard) => { println!("Valore letto con successo: {:?}", guard);  }
            Err(poison_error) => {
                // Gestiamo l'errore derivante dal Mutex in stato "poisoned"
                println!("Reader 2 - errore: il Mutex è stato avvelenato");}
        }
    });
    let writer = thread::spawn(move || {// Thread in scrittura (che provoca uno stato "poisoned")
        let mut guard = data_clone3.write().unwrap();
        guard.push(4); // Prova a inserire un elemento nella struttura dati
        panic!("Oops, ho fatto un errore!"); // Simula un errore (ad esempio, un panic)
    });
    reader1.join().unwrap();
    reader2.join().unwrap();
    writer.join().unwrap_err();
}
