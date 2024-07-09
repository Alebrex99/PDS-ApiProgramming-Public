use std::io::{Read, Write};
use std::process::{Command, Stdio};
use rand::Rng;
fn main() {
   let mut child = Command::new("client") // Avvia il processo figlio (client)
        .env("PATH", "./target/debug/")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start child process");

    let mut rng = rand::thread_rng();

    let mut pipe_reader = child.stdout.take().expect("Failed to open stdout");
    let mut pipe_writer = child.stdin.take().unwrap();

    let mut number_str = [0; 2];
    pipe_reader.read_exact(&mut number_str).expect("Failed to read number");

    println!("Il client è vivo!");
loop {
        // Invia un acknowledge al client
        pipe_writer.write_all("ACK\n".as_bytes()).expect("Failed to send acknowledge");

        let mut number_str = [0; 2];
        pipe_reader.read_exact(&mut number_str).expect("Failed to read number");
        let number = std::str::from_utf8(&number_str).unwrap().trim();
        println!("Ricevuto dal client:{}", number);
        let random_number = rng.gen_range(0..=9);

        if random_number == number.parse::<i32>().unwrap(){
            pipe_writer.write_all("STOP\n".as_bytes()).expect("Failed to send acknowledge");
            break;
        }

    }
    // Attendere che il processo figlio termini
    child.wait().expect("Failed to wait for child process");
}
