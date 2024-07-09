fn main() {
    let boxed_slice: Box<[i32]> = Box::new([1, 2, 3, 4, 5]);

    // Creare una slice che considera solo una parte del vettore contenuto nel Box
    let slice: &[i32] = &boxed_slice[1..3]; // Da 2 a 3 (escluso 3)

    println!("{:?}", slice); // Stampa: [2, 3]
}