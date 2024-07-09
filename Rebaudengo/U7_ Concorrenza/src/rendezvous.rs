use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::sync_channel(0);

    // Questo thread tenta di inviare un valore al canale, ma essendo la capacità 0,
    // si bloccherà fino a quando il valore non verrà letto dal receiver.
    thread::spawn(move || {
        sender.send("Sto trasmettendo un messaggio e ci aspettiamo 								all'appuntamento").unwrap();
        println!("Valore inviato.");
    });

    // Il receiver legge il valore dal canale.
    let received_value = receiver.recv().unwrap();
    println!("Valore ricevuto: {}", received_value);
}
