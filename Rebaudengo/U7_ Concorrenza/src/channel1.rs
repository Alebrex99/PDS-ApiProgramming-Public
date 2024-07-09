


use std::sync::mpsc::{channel};
use std::thread;

fn main() {
    let (tx, rx) = channel();

    for _ in 0..3 {
        let tx = tx.clone();
        // cloned tx dropped within thread
        thread::spawn(move || tx.send("ok").unwrap());
    }

// Drop the last sender to stop `rx` waiting for message.
// The program will not complete if we comment this out.
// **All** `tx` needs to be dropped for `rx` to have `Err`.
    drop(tx);

// Unbounded receiver waiting for all senders to complete.
    while let Ok(msg) = rx.recv() {
        println!("{}", msg);
    }
}