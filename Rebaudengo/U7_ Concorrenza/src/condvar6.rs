use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;
struct SharedData{
    mutex:Mutex<bool>,
    cv:Condvar
}
impl SharedData{
    //Metodo costruttore
    pub fn new(condition:bool)->Self{
        SharedData { mutex:Mutex::new(condition), cv:Condvar::new() }
    }
    pub fn change_and_notify(&self){
        let mut data=self.mutex.lock().unwrap();
        *data = true;
        //Mandiamo la notifica alla condvar
        self.cv.notify_one();
    }
    pub fn looper(&self){
        loop {
            //Il thread aspetta per una notifica. Nel caso siano passati 100 misecondi smette di aspettare
            let lock= self.cv.wait_timeout(
                self.mutex.lock().unwrap(), Duration::from_millis(100)).unwrap();
            if *lock.0 == true {
                //Il thread ha ricevuto una notifica dato che il valore è stato cambiato, quindi esco
                print!("Il valore è cambiato quindi posso uscire dal ciclo correttamente ");
                if !lock.1.timed_out() {
                    println!("E non c'è stato time-out");
                }
                break
            }
            if lock.1.timed_out() {
                println!("E' scaduto il timer ma il valore non è cambiato. Ricomincio il ciclo");
            }
        }
    }
}
fn main(){
    let shared = Arc::new(SharedData::new(false));
    let shared2 = Arc::clone(&shared);


    let mut handles = vec![]; //vettore dei threads creati per poi fare le dovute join

    handles.push(thread::spawn(move|| {
        shared2.looper();
    }));

    handles.push(thread::spawn(move|| {
        //Aspetto prima di mandare la notifica
        thread::sleep(Duration::from_secs(1));
        shared.change_and_notify();
    }));

    //Join finali
    for handle in handles {
        handle.join().expect("Error");
    }
}

