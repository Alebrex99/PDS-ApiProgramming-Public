fn main() {
    let mut count = 0;

    let mut increment_n =  |n| {
        count += n; // Incrementiamo la variabile catturata
        println!("Il conteggio è: {}", count);
    };

    increment_n(10);

    // println!("{}", count);

    increment_n(5);

    // println!("{}", count);
}