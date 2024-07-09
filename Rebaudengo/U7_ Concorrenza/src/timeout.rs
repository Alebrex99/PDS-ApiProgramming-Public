use crossbeam_channel::select;
use crossbeam::channel;
use std::thread;
use std::time::{Duration, Instant};
fn main() {
    let (s, r) = channel::bounded(1);
    let after = channel::after(Duration::from_secs(5)); // Timeout dopo 5 secondi
    thread::spawn(move || {
        // Simula un'operazione di calcolo intensivo
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(8) {
            // Fa qualcosa di computazionalmente intenso
        }
        let _ = s.send("Operazione completata").unwrap();
    });
    select! {
        recv(r) -> msg => println!("{}", msg.unwrap()), // Riceve il messaggio
        recv(after) -> _ => println!("Timeout! Operazione non completata"), // Timeout
    }
    let (s, r) = channel::bounded(1);
    let after = channel::after(Duration::from_secs(5)); // Timeout dopo 5 secondi
    thread::spawn(move || {
        // Simula un'operazione di calcolo intensivo
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(1) {
            // Fa qualcosa di computazionalmente intenso
        }
        let _ = s.send("Operazione completata").unwrap();
    });
    select! {
        recv(r) -> msg => println!("{}", msg.unwrap()), // Riceve il messaggio
        recv(after) -> _ => println!("Timeout! Operazione non completata"), // Timeout
    }
}

