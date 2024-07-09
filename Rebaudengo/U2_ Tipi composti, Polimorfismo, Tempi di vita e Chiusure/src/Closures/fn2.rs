fn main() {
    // Una chiusura che accetta un parametro e implementa il tratto Fn
    let mut x = 10;

    let aggiungi_a_x = |y: i32| {
        println!("Il risultato di x + y è: {}", x + y);
    };

    aggiungi_a_x(5); // Chiamiamo la chiusura con il parametro 5
    aggiungi_a_x(7); // Chiamiamo la chiusura con il parametro 7
    aggiungi_a_x(7);  // Chiamiamo la chiusura con il parametro 7 
}
