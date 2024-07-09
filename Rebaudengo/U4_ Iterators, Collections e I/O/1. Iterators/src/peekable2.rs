fn main() {
    let numeri = vec![1, 2, 3, 4];
    let mut iteratore_peekable = numeri.iter().peekable();

    while let Some(numero) = iteratore_peekable.next() {
        println!("Valore dall'iteratore: {}", numero);
        match iteratore_peekable.peek() {
            Some(prossimo) => println!("Sbirciando il prossimo valore: {}", prossimo),
            None => println!("Non ci sono altri valori"),
        }
    }
}
