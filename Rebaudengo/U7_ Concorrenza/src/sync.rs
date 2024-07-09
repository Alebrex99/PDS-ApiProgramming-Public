fn main() {
    // Creiamo una variabile condivisa tra più thread.
    let shared_value = 42;

    // Creiamo due thread che accedono alla variabile condivisa.
    let handle1 = std::thread::spawn(move || {
        println!("Thread 1: Valore condiviso = {}", shared_value);
        let a = shared_value + 1;
        println!("Thread 1: Valore calcolato = {}", a);
    });

    let handle2 = std::thread::spawn(move || {
        println!("Thread 2: Valore condiviso = {}", shared_value);
        let b = shared_value + 10;
        println!("Thread 2: Valore calcolato = {}", b);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
