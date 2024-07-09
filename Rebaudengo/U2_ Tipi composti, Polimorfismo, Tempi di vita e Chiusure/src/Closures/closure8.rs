fn main() {
    let numbers = vec![1, 2, 3, 4, 5]; 

    let print_numbers = move || {
        println!("I numeri sono: {:?}", numbers); 
        // La chiusura usa il vettore catturato
        // numbers è stato spostato (moved) dentro la chiusura
    };

    // non può più essere usato qui
    //println!("I numeri sono: {:?}", numbers);
    // Questo darebbe errore di compilazione

    print_numbers(); // Chiamiamo la chiusura
}
