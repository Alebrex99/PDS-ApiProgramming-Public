
use std::collections::BTreeSet;
use std::ops::Bound::{Included, Excluded};

fn main() {
    // Creare un BTreeSet con alcuni numeri
    let mut numeri: BTreeSet<i32> = BTreeSet::new();
    numeri.insert(5);
    numeri.insert(10);
    numeri.insert(15);
    numeri.insert(18);
    numeri.insert(20);
    numeri.insert(25);

    // Usare il metodo range per ottenere un iteratore su un intervallo di valori
    // In questo caso, tutti i numeri maggiori o uguali a 10 e minori di 20
    for numero in numeri.range(10..20) {
        println!("Numero nell'intervallo: {}", numero);
    }
}