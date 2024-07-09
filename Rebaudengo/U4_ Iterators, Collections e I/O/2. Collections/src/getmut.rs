fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    // Modifichiamo il secondo elemento (indice 1) del vettore
    if let Some(second_element) = numbers.get_mut(1) {
        *second_element = 10;
    }

    // Stampa il vettore modificato
    println!("Vettore dopo la modifica: {:?}", numbers);
}