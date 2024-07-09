fn main() {
    let numbers1 = vec![1, 2, 3];
    let numbers2 = vec![4, 5, 6, 7];

    // Concateniamo i due vettori utilizzando il metodo chain
    let chained_numbers = numbers1
            .iter()
            .chain(numbers2.iter());
    for n in chained_numbers {
        println!("Numeri concatenati: {:?}", n);
        }
}
