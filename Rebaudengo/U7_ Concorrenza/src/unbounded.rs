use std::thread;
use crossbeam_channel::{unbounded};

fn main() {
    let (s, r) = unbounded();
    let mut vettore: Vec<i32> = Vec::new();

    thread::spawn(move || {

    // Can send any number of messages into the channel without blocking.
        for i in 0..50 {
            s.send(i).unwrap();
        }
        drop(s); // Disconnect the channel.
    });

    while let Ok(result) = r.recv() {
        vettore.push(result);
    }
    println!("{:?}", vettore);
}

