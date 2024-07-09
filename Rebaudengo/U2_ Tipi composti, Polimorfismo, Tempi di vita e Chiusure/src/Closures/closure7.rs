fn main() {
    let mut count = 0; 

    let mut increment = move || {
        count += 1; // Incrementiamo la variabile catturata
        println!("Il conteggio è: {}", count);
    };

    increment();

    increment();
}
