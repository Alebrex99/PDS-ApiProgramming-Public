
use std::result::Result;
use std::sync::mpsc::{channel, Receiver, Sender, SendError};
use std::sync::Mutex;
use std::thread;

pub struct MultiChannel {
    channels: Mutex<Vec<Sender<u8>>>,
}

impl MultiChannel {
    fn new() -> Self {
        MultiChannel {
            channels: Mutex::new(Vec::new()),
        }
    }

    fn send(&self, data: u8) -> Result<(), SendError<u8>> {
        let mut channels = self.channels.lock().unwrap();
        if channels.is_empty() {
            return Err(SendError(data));
        }
        channels.retain(|sender| sender.send(data).is_ok());
        if channels.is_empty() {
            Err(SendError(data))
        } else {
            Ok(())
        }
    }

    fn subscribe(&self) -> Receiver<u8> {
        let (tx, rx) = channel();
        let mut channels = self.channels.lock().unwrap();
        channels.push(tx);
        rx
    }
}
fn main() {
    let multi_channel = MultiChannel::new();

    // Subscriber 1
    let rx1 = multi_channel.subscribe();
    thread::spawn(move || {
        while let Ok(data) = rx1.recv() {
            println!("Subscriber 1 received: {}", data);
        }
    });

    // Subscriber 2
    let rx2 = multi_channel.subscribe();
    thread::spawn(move || {
        while let Ok(data) = rx2.recv() {
            println!("Subscriber 2 received: {}", data);
        }
    });

    // Send data
    multi_channel.send(10).unwrap();
    multi_channel.send(20).unwrap();
    multi_channel.send(30).unwrap();

    // Wait for a moment to ensure all messages are received
    thread::sleep(std::time::Duration::from_secs(1));
}
