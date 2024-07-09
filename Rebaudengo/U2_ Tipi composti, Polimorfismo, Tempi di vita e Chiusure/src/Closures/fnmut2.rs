fn main() {
    let mut x = 10;

    // Una chiusura che implementa il tratto FnMut e accetta un parametro
    let mut aggiungi_a_x = |y: i32| {
        x += y;
        println!("Il nuovo valore di x è: {}", x);
    };

    aggiungi_a_x(5); // Chiamiamo la chiusura con il parametro 5
    aggiungi_a_x(7); // Chiamiamo la chiusura con il parametro 7
}
