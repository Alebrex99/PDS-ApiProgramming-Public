use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 4];
    stream.read_exact(&mut buffer).expect("Errore nella lettura del buffer");

    let num = i32::from_be_bytes(buffer);

    println!("Ricevuto {}", num);
    let doubled = num * 2;

    stream.write_all(&doubled.to_be_bytes()).expect("Errore nell'invio della risposta");
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Errore nel binding del listener");
    println!("Server in ascolto su 127.0.0.1:8080...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Errore nell'accettare la connessione: {}", e);
            }
        }
    }
}
