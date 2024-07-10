use std::sync::mpsc::{channel, Receiver, Sender, SendError};
use std::sync::Mutex;

//MULTICHANNEL
pub struct MultiChannel{
    senders: Mutex<Vec<Sender<u8>>>
}

impl MultiChannel{
    pub fn new() -> Self{
        //crea un canale senza ricevitori collegati
        MultiChannel{senders: Mutex::new(Vec::new())}
    }
    pub fn subscribe(&self) -> Receiver<u8>{
        let (tx, rx) = channel::<u8>();
        let mut senders = self.senders.lock().unwrap();
        senders.push(tx);
        rx //movimento nel main() di tale rx
        /*se il Receiver restituito viene drop(rx) nel main, comunque il canale continua a funzionare
        perchè tx è vivo visto che l'ho salvato in VEC di struttura dati. rx se eliminato nel main non ci sarà più*/
    }
    pub fn send(&self, data:u8) -> Result<(), SendError<u8>>{ //deve restituire il risultato dell'invocazione di tx.send()
        let mut senders = self.senders.lock().unwrap();
        senders.retain(|sender| sender.send(data).is_ok()); //elimini i sender che non hanno più ricevitori
        if senders.is_empty(){
            Err(SendError(data))
        }
        else {
            Ok(())
        }
    }
}