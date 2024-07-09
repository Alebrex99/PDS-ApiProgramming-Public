use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn start_process(sender: Sender<String>, receiver: Receiver<String>) {

    let child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");
    thread::spawn(move || {
        let mut pipe_in = BufReader::new(child.stdout.unwrap());
        let mut pipe_out = child.stdin.unwrap();
        for line in receiver {
            pipe_out.write_all(line.as_bytes()).unwrap();
            let mut buf = String::new();
            match pipe_in.read_line(&mut buf) {
                Ok(_) => {
                    // inoltra quanto ricevuto dalla pipe sul canale di uscita
                    sender.send(buf).unwrap();
                    continue;
                }
                Err(e) => {
                    println!("an error!: {:?}", e);
                    break;
                }
            }
        }
    });
}
fn start_command_thread(sender: Sender<String>) {
    thread::spawn(move || {
        for i in 1..10 {
            sleep(Duration::from_secs(3));
            sender.send(String::from(format!("Message {} from command thread\n", i)))
                .unwrap();
        }
    });
}

fn main() {
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    start_process(tx1, rx2);

    start_command_thread(tx2);

    rx1.iter().for_each(|line| println!("Echo process response: {}", line))
}







