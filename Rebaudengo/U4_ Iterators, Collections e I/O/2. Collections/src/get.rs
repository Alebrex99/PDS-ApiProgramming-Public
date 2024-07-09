fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Otteniamo una referenza all'elemento al secondo indice (indice 1)
    if let Some(second_element) = numbers.get(1) {
        println!("Il secondo elemento è: {}", second_element);
    } else {
        println!("Il secondo elemento non esiste nel vettore");
    }

    // Otteniamo una referenza all'elemento al sesto indice (indice 5)
    if let Some(sixth_element) = numbers.get(5) {
        println!("Il sesto elemento è: {}", sixth_element);
    } else {
        println!("Il sesto elemento non esiste nel vettore");
    }
}
