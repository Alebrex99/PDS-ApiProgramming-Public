fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Somma tutti gli elementi della collezione
    let sum = numbers.iter().fold(0, |accumulatore, &x| accumulatore + x);

    println!("La somma di tutti gli elementi è: {}", sum);
}
