fn main() {
    let numbers = vec![1, 2, 3];
    let mut iter = numbers.iter().fuse();

    // Consuma l'iteratore completamente
    println!("Iteratore 1:");
    while let Some(&number) = iter.next() {
        println!("Numero: {}", number);
    }

    // Riprova a consumare l'iteratore
    println!("Iteratore 2:");
    while let Some(&number) = iter.next() {
        println!("Numero: {}", number);
    }
}
