use std::sync::Mutex;
use std::sync::Arc;
use std::thread;
fn main() {
    // Creiamo un vettore condiviso protetto da un mutex
    let shared_data = Arc::new(Mutex::new(Vec::new()));

    // Cloniamo l'Arc per poterlo condividere tra più thread
    let shared_data_clone = Arc::clone(&shared_data);

    // Creiamo un thread per aggiungere dati al vettore condiviso
    let handle1 = thread::spawn(move || {
        let mut data = shared_data_clone.lock().unwrap();
        data.push(1);
    });
    // Cloniamo nuovamente l'Arc per un altro thread
    let shared_data_clone = Arc::clone(&shared_data);
    // Creiamo un secondo thread per aggiungere dati al vettore condiviso
    let handle2 = thread::spawn(move || {
        let mut data = shared_data_clone.lock().unwrap();
        data.push(2);
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("{:?}", shared_data.lock().unwrap()); // Stampa il vettore condiviso 
}

