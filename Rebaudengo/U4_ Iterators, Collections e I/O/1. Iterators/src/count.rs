fn main() {
    let numbers = vec![2, 4, 6, 7, 8, 9];

    let odd = numbers.iter().filter(|&x| *x % 2 != 0).count();

    println!("Il numero di dispari è: {}", odd);

}
