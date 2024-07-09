fn main() {
    let numbers = vec![1, 2, 3];

    // Otteniamo un iteratore che cicla ripetutamente attraverso i numeri
    let mut cycle_iter = numbers.iter().cycle();

    // Stampiamo i primi 5 numeri del ciclo
    for _ in 0..5 {
        if let Some(num) = cycle_iter.next() {
            println!("Numero: {}", num);
        }
    }
}
