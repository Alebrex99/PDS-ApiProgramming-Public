fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let index = 5;

    if index < numbers.len() {
        let value = numbers[index];
        println!("Il valore all'indice {} è: {}", index, value);
    } else {
        panic!("Tentativo di accedere a un indice non valido.");
    }
}