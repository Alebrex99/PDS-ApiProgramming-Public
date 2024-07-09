use std::cell::Cell;
use std::thread;

fn main() {
    // Crea un Cell contenente un intero
    let my_cell = Cell::new(42);

    // Clona il Cell per entrambi i thread
    let thread1_cell = my_cell.clone();
    let thread2_cell = my_cell.clone();

    // Thread 1: Incrementa il valore nel Cell
    let handle1 = thread::spawn(move || {
        thread1_cell.set(thread1_cell.get() + 1);
        println!("Thread1: Valore nel Cell: {}", thread1_cell.get());
    });

    // Thread 2: Legge il valore dal Cell
    let handle2 = thread::spawn(move || {
        println!("Thread2: Valore nel Cell: {}", thread2_cell.get()-1);
    });
    println!("Main Thread: Valore nel Cell: {}", my_cell.get());
    // Attendi che entrambi i thread terminino
    handle1.join().unwrap();
    handle2.join().unwrap();
}
