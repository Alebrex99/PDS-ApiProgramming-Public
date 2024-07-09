use std::thread;

// Definiamo una struttura condivisa tra thread
#[derive(Clone, Copy)]
struct SharedData {
    value: u32,
}

fn main() {
    // Creiamo una variabile condivisa tra thread
    let shared_data = SharedData { value: 42 };

    // Creiamo un thread che accede alla variabile condivisa
    let handle = thread::spawn(move || {
        println!("Il valore condiviso è: {}", shared_data.value);
    });
    println!("Il valore condiviso è: {}", shared_data.value);
    // Attendiamo il completamento del thread
    handle.join().unwrap();
}
