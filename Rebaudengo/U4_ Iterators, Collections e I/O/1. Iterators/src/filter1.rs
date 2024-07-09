fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];


// Utilizzo di filtri per selezionare solo i numeri pari
let even_numbers = numbers.iter().filter(|&x| x % 2 == 0);

// Stampiamo i numeri pari
for n in even_numbers {
	println!("Even number: {}", n);
    }
}
