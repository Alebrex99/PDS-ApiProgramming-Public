fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let even_numbers: Vec<_> = numbers.into_iter().filter(|&x| x % 2 == 0).collect();
    println!("{:?}", even_numbers); // Stampa: [2, 4, 6]
}