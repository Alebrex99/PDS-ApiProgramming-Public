fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Stampiamo ogni numero del vettore
    numbers.iter().for_each(|&num| {
        println!("Numero: {}", num);
    });
}
