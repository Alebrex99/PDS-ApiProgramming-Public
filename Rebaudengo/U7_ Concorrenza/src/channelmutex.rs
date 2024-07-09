
//Utilizziamo entrambi i modelli di concorrenza per la sincronizzazione di thread-> sync_channel e mutex
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug)]
pub struct Task {
    id: usize,
    testo: String,
}
fn main() {
    //Definisco un canale con il quale dialogare, in cui vi sono più producer e 1 consumer
    let (task_sender, task_receiver) = channel::<Task>();

    // Per condividere il risultato tra thread in mutua esclusione, ed essenziallmente per tener traccia di quante task
    //sono state effettuate
    let result_counter = Arc::new(Mutex::new(0));

    // Faccio partire 2 thread
    for i in 0..2 {
        let tx = task_sender.clone();
        thread::spawn(move || {
            for j in 0..5 {
                let task = Task {
                    id: i * 5 + j,
                    testo: format!("Task {} from Producer {}", i * 5 + j, i),
                };
                tx.send(task).unwrap();//invio la task
                thread::sleep(Duration::from_millis(200));//Aspetto per renderlo più deterministico
            }
        });
    }
    let result_counter_t = Arc::clone(&result_counter);
    thread::spawn(move || {
        loop {
            //Per ricevere il dato e poi printarlo mi metto in loop sul receiver
            match task_receiver.recv() {
                Ok(task) => {
                    println!("Consumer received task: {:?}", task);

                    thread::sleep(Duration::from_millis(400));

                    // Incremento il mutex per tener traccia delle task effettuate
                    {
                        let mut counter = result_counter_t.lock().unwrap();
                        *counter += 1;
                    }
                }
                Err(_) => break, //Esco dal loop quando il channel si chiude
            }
        }
    });

    // Aspetto che finiscano tutti i sender
    thread::sleep(Duration::from_secs(3));
    // Chiudo il canale con il drop del sender
    drop(task_sender);

    thread::sleep(Duration::from_secs(1));

    let result_counter = Arc::clone(&result_counter);

    // Stampo il numero di task facendo lock sul mutex
    let counter = result_counter.lock().unwrap();
    println!("Total tasks processed: {}", counter);
}