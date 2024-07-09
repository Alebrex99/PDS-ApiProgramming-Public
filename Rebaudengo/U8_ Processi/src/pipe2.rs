use std::fs::File;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::thread;

fn main() -> io::Result<()> {
    // Nome del file da leggere
    let filename = "test.txt";

    // Creare un thread per leggere il contenuto del file
    let handle = thread::spawn(move || -> io::Result<String> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    });

    // Creare un processo per eseguire il comando rev
    let mut child = Command::new("rev")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    // Attendere che il thread finisca e ottenere il contenuto del file
    let file_contents = handle.join().unwrap()?;

    // Ottenere lo stdin del processo e scrivere il contenuto del file
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(file_contents.as_bytes())?;
    }

    // Ottenere lo stdout del processo e leggere l'output
    let output = child.wait_with_output()?;

    // Stampare l'output del comando rev
    println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}