fn main() {
    let mut vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    vec1.extend(vec2);

    println!("{:?}", vec1); // Output: [1, 2, 3, 4, 5, 6]
}
