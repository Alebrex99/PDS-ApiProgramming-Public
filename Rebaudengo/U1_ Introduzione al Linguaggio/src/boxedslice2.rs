fn main() {

    let boxed_slice: Box<[i32]> = Box::new([1, 2, 3, 4, 5]);
    let vec: Vec<i32> = boxed_slice.into_vec();

    println!("{:?}", vec);
}