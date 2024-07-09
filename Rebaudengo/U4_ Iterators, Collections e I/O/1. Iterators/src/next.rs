fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    // Ottiene il primo elemento dall'iteratore
    if let Some(first_num) = numbers.iter().next() {
        println!("Il primo numero e' {} ", first_num);

    } else {
        println!("Nessun elemento trovato");
    }

    if let Some(first_num) = numbers.iter_mut().next() {
        *first_num += 1;

    } else {
        println!("Nessun elemento trovato");
    }
    for num in numbers.iter() {
        println!("{}", num);
    }
}
