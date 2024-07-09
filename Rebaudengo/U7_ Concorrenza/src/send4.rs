fn main() {
    // Creiamo una variabile condivisa tra thread
    let mut shared_data = SharedData { value: 42 };

    // Creiamo un thread che accede alla variabile
    let handle = thread::spawn(move || {
        shared_data.value += 1;
        println!("Il valore è: {}", shared_data.value); // Output: 43
    });

    // Attendiamo il completamento del thread
    handle.join().unwrap();
    println!("Il valore è: {}", shared_data.value); // Output: 42
}





