use std::thread;
use std::time::Duration;

fn main() {
    // Creiamo un nuovo thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Ciao numero {} dal thread creato!", i);
            thread::sleep(Duration::from_millis(1));
        }
        // Restituiamo un valore dal thread
        "OK".to_string()
    });

    for i in 1..5 {
        println!("Ciao numero {} dal thread principale!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Attendiamo che il thread creato finisca e otteniamo il valore di ritorno
    match handle.join() {
        Ok(res) => {println!("Terminazione corretta {}", res);},
        Err(err) => {println!("Terminazione errata {:?}", err);}

    }
}