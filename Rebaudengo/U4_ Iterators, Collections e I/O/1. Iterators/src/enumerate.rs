fn main() {
    let my_vec = vec!["a", "b", "c"];

    // Otteniamo un iteratore che produce coppie (indice, valore)
    let mut iter = my_vec.iter().enumerate();

    // Iteriamo attraverso ogni coppia (indice, valore)
    while let Some((index, value)) = iter.next() {
        println!("Indice: {}, Valore: {}", index, value);
    }
}
