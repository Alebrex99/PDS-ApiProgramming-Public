
use std::thread;
use std::time::Duration;
use crossbeam::channel;

fn main() {
    let (s, r) = channel::unbounded();
    let tick = channel::tick(Duration::from_secs(1));
    // Genera un impulso ogni secondo

    let mut counter = 0;
    thread::spawn(move || {
        for _ in 0..5 {
            let _ = tick.recv(); // Riceve l'impulso
            s.send("Impulso ricevuto").unwrap();
        }
    });

    for _ in 0..5 {
        println!("{}", r.recv().unwrap());
        // Stampa "Impulso ricevuto" ogni secondo
        counter += 1;
    }
    println!("Ho ricevuto {} impulsi", counter);
}