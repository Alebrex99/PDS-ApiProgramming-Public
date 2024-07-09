fn main() {
    let boxed_slice: Box<[i32]> = Box::new([1, 2, 3, 4, 5]);

    // Ottenere una slice dall'Box<[i32]>
    let slice: &[i32] = &*boxed_slice;

    println!("{:?}", slice); // Stampa: [1, 2, 3, 4, 5]
}