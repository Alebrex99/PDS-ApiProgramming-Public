fn main() {
    let nested_numbers = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    // Appiattisce la struttura nested di vettori in un singolo vettore
    let flattened_numbers = nested_numbers.into_iter().flatten();

    for n in flattened_numbers {
        println!("Numeri appiattiti: {:?}", n);
    }
}
