use std::sync::mpsc::sync_channel;
use std::thread;

fn main() {


    let (sender, receiver) = sync_channel(1);
    // this returns immediately

    sender.send(1).unwrap();
    thread::spawn(move|| {
        // this will block until the previous message has been received
        sender.send(2).unwrap();
    });

    println!("{}", receiver.recv().unwrap());
    println!("{}", receiver.recv().unwrap());
}
