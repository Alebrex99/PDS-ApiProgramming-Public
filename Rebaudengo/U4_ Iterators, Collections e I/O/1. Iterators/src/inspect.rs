fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Utilizziamo inspect per stampare ogni elemento prima di moltiplicarlo per 2
    let doubled_numbers = numbers.iter()
        .inspect(|&x| println!("Elemento: {}", x))
        .map(|&x| x * 2);

    for n in doubled_numbers {
        println!("Raddoppiato: {:?}", n);
    }
}