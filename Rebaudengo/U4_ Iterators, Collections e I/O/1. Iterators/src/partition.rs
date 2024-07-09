fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Dividi la collezione in numeri pari e dispari
    let (even_numbers, odd_numbers): (Vec<_>, Vec<_>) = 	
	numbers	
	.into_iter()
	.partition(|&x| x % 2 == 0);

    println!("Numeri pari: {:?}", even_numbers);
    println!("Numeri dispari: {:?}", odd_numbers);
}
