use std::thread;

fn main() {
    let data = vec![1, 2, 3];

    // Creiamo un nuovo thread e trasferiamo il possesso del vettore "data"
    let handle = thread::spawn(move || {
        let somma: i32 = data.iter().sum();
        println!("Ecco un vettore: {:?} la cui sommatoria è {}", data, somma);

    });
    println!("Il thread principale non ha accesso ai dati !");

    // println!("Ecco un vettore: {:?}", data);
    // Attendiamo che il thread creato finisca
    match handle.join() {
        Ok(_) => {println!("Terminazione corretta");},
        Err(err) => {println!("Terminazione errata {:?}", err);}

    }
}