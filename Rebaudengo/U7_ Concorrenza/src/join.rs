use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4];
    // provare con 
    // let data = vec![];
    // e vedere che cosa succede

    // Creiamo un nuovo thread e trasferiamo il possesso del vettore "data"
    let handle = thread::spawn(move || {
        let len:usize= data.len();
        let somma: usize = data.iter().sum();
        somma/len

    });
    println!("Il thread principale aspetta");
    let average = handle.join();
    match average {
        Ok(res) => {println!("La media è {:?}", res);},
        Err(err) => {println!("Errore {:?}", err);}
    }
    println!("Il thread principale termina.");
}
