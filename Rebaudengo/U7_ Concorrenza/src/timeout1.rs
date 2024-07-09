
use crossbeam_channel::select;
use crossbeam::channel;
use std::thread;
use std::time::{Duration, Instant};
fn main() {
    let (s, r) = channel::bounded(1);
    let after = channel::after(Duration::from_secs(5)); // Timeout dopo 5 secondi

    thread::spawn(move || {
        for i in 0..10    {
        // Simula un ciclo di operazioni di calcolo intensivo
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(i) {
            // Fa qualcosa di computazionalmente intenso
        }
        let _ = s.send("Operazione completata").unwrap();
        }
    });
    loop {
        select! {
        recv(after) -> _ => {
                println!("Timeout! Operazione non completata"); // Timeout
                    break;
            }
        recv(r) -> msg => println!("{}", msg.unwrap()), // Riceve il messaggio
        }

    }
}
