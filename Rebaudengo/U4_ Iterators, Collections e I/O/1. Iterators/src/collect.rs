fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Filtriamo solo i numeri pari e li raccogliamo in un nuovo vettore
    let even_numbers: Vec<_> = numbers.iter()
        .filter(|&x| x % 2 == 0)
        .collect();

    println!("Numeri pari: {:?}", even_numbers);
}




