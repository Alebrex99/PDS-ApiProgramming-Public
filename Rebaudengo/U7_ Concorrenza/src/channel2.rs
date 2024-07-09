use std::sync::mpsc::{Sender, channel, Receiver};
use std::thread;
struct SharedMsg{
    tx:Sender<String>,
    rx:Receiver<String>
}
impl SharedMsg{
    pub fn new()->Self{
        //Creazione del canale
        let (tx,rx)=channel::<String>();
        SharedMsg { tx: tx, rx: rx }
    }
}
fn main() {
    let mut handles = vec![];
    let shared=SharedMsg::new();
    let tx1=shared.tx.clone();
    handles.push(thread::spawn(move|| {
        if tx1.send("Ciao".to_string()).is_err()==true { println!("Errore nell'invio del messaggio"); }
    }));
    let tx2=shared.tx.clone();
    handles.push(thread::spawn(move|| {
        if tx2.send("Come stai?".to_string()).is_err()==true{ println!("Errore nell'invio del messaggio"); }
    }));
    // Occorre chiudere il canale,
    // Senza questo il receiver continuerebbe a rimanere in attesa di messaggi
    drop(shared.tx);
    while let Ok(msg)=shared.rx.recv(){
        println!("Messaggio ricevuto:");
        println!("{}",msg);
    }
    for handle in handles {
        handle.join().expect("Error");
    }
}
