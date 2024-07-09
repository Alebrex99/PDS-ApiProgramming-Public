/*
Exchanger è una struttura che funziona solo a coppie
pub fn make_exchangers() -> (Exchanger<T>, Exchanger<T>)  crea due exchanger
pub fn exchange(&self, msg: T) -> Option<T> è bloccante finchè non viene invocato il suo metodo gemello
e restituisce il valore passato come argomento al metodo gemello
*/

use std::sync::Mutex;

struct Exchanger<T: Send> {
    tx: Mutex<std::sync::mpsc::SyncSender<T>>,
    rx: Mutex<std::sync::mpsc::Receiver<T>>,
}
//Soluzione mia
impl<T: Send> Exchanger<T> {
    pub fn make_exchangers() -> (Exchanger<T>, Exchanger<T>) {
        let (tx1, rx1) = std::sync::mpsc::sync_channel(1);
        let (tx2, rx2) = std::sync::mpsc::sync_channel(1);
        (
            Exchanger {
                tx: Mutex::new(tx1),
                rx: Mutex::new(rx2),
            },
            Exchanger {
                tx: Mutex::new(tx2),
                rx: Mutex::new(rx1),
            },
        )
    }
    pub fn exchange(&self, msg: T) -> Option<T> {
        let tx_guard = self.tx.lock().unwrap();
        let rx_guard = self.rx.lock().unwrap();
        let a = tx_guard.send(msg);
        match a {
            Ok(_) => (),
            Err(_) => return None,
        };
        match rx_guard.recv() {
            Ok(elem) => return Some(elem),
            Err(_) => return None,
        }
    }
}
//Codice per testare Exchanger
pub fn driver_exchanger() {
    let (ex1, ex2) = Exchanger::<i32>::make_exchangers();
    let a = std::thread::spawn(move || {
        let b = ex1.exchange(10);
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("I'm t1 and I sent 10 {:?}", b);
    });
    let c = std::thread::spawn(move || {
        let d = ex2.exchange(20);
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("I'm t2 and I sentt 20 {:?}", d);
    });
    a.join().unwrap();
    c.join().unwrap();
}
