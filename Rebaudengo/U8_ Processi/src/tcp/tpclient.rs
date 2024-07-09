use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Errore nella connessione al server");

    let num: i32 = 42; // Numero da inviare al server
    stream.write_all(&num.to_be_bytes()).expect("Errore nell'invio del numero");

    let mut buffer = [0; 4];
    stream.read_exact(&mut buffer).expect("Errore nella lettura della risposta");

    let result = i32::from_be_bytes(buffer);
    println!("Risposta dal server: {}", result);
}
