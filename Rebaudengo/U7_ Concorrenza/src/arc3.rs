use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let shared_data = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];
    for i in 1..10 {
        let data = shared_data.clone();    //duplicazione del possesso
        threads.push( thread::spawn( move || { //data è ceduto al thread
            let mut v = data.lock().unwrap();  //v è di tipo MutexGuard<T>
            println!("{:?}", i);
            v.push(i);                      //quando v esce dall scope, il lock
        }) );                               //viene rilasciato
    }
    for t in threads { t.join().unwrap(); }

    //v contiene i numeri da 1 a 9
    println!("\nResult: {:?}", *(shared_data.lock().unwrap()));
}

