fn main() {
    let numbers = vec![1, 2, 3];
    let mut iter = numbers.iter().fuse();

    // Stampa i primi due numeri
    println!("First: {:?}", iter.next());  // Some(1)
    println!("Second: {:?}", iter.next()); // Some(2)
    println!("Third: {:?}", iter.next());  // Some(3)

    // Stampa None e Continua a restituire None
    println!("Fourth: {:?}", iter.next()); // None
    println!("Fifth: {:?}", iter.next());  // None
}
