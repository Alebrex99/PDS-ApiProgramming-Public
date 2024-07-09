fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    // Incrementa ogni numero nel vettore di 1 utilizzando iter_mut()
    for num in numbers.iter_mut() {
        *num += 1;
    }

    // Stampa il vettore dopo l'incremento
    println!("{:?}", numbers); // Output: [2, 3, 4, 5, 6]

    // Incrementa ogni numero nel vettore di 1 utilizzando &mut
    for num in &mut numbers {
        *num += 1;
    }

    // Stampa il vettore dopo l'incremento
    println!("{:?}", numbers); // Output: [3, 4, 5, 6, 7]
}
