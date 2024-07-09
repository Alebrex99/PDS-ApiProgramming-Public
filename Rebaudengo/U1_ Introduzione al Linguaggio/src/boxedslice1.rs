fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    let boxed_slice: Box<[i32]> = vec.into_boxed_slice();

    println!("{:?}", boxed_slice);
}
