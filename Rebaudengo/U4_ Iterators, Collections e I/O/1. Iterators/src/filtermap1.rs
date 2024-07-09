fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Utilizzo di filter_map per filtrare e mappare i numeri pari
    let even_numbers= numbers
        .iter()
        .filter_map(|&x| {
            if x % 2 == 0 {
                Some(x)
            } else {
                None
            }
        });

    for n in even_numbers {
        println!("{:?}", n);
    } // Stampa: 2, 4
}
