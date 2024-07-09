fn main() {
    let numbers = vec![1, 2, 3];
    let letters = vec!['a', 'b', 'c'];

    // Otteniamo un iteratore che produce tuple (numero, lettera)
    let mut iter = numbers.iter().zip(&letters);

    // Iteriamo attraverso ogni coppia (numero, lettera)
    while let Some((number, letter)) = iter.next() {
        println!("Numero: {}, Lettera: {}", number, letter);
    }
}
