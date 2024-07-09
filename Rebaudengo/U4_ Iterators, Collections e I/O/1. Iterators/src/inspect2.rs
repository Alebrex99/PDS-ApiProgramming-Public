fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Utilizziamo inspect per controllare se un elemento è pari o dispari
    let parity_check= numbers.iter()
        .inspect(|&x| {
            if x % 2 == 0 {
                println!("{} è pari", x);
            } else {
                println!("{} è dispari", x);
            }
        });

    for n in parity_check {
        println!("Numero: {:?}", n);
    }
}