use std::thread;
use crossbeam_channel::{bounded, Receiver, Sender};
fn worker(id: usize, rx: Receiver<i32>, tx: Sender<String>) {
    while let Ok(value) = rx.recv() {
        tx.send(format!("W{} ({})", id, value)).unwrap();
    }
}

fn main() {
    let (tx_input, rx_input) = bounded::<i32>(10);
    let (tx_output, rx_output) = bounded::<String>(10);
    let mut worker_handles = Vec::new();
    for i in 0..3 {
        let rx = rx_input.clone();
        let tx = tx_output.clone();
        worker_handles.push( thread::spawn(move || worker(i, rx, tx)) );
    }
    for i in 1..=10
    {
        tx_input.send(i).unwrap(); 
    }
    drop(tx_input);
    drop(tx_output);

    while let Ok(result) = rx_output.recv() {
        println!("Received result: {}", result);
    }
    for handle in worker_handles { handle.join().unwrap(); }
}

