fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Calcola la somma di tutti gli elementi nell'iteratore
    let sum: i32 = numbers.iter().sum();

    println!("La somma di tutti gli elementi è: {}", sum);
}
