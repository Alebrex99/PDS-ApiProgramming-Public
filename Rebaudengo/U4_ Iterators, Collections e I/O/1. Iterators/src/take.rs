fn main() {
        	let numbers = vec![1, 2, 3, 4, 5];

        	// Prende solo i primi 3 numer i dall'inizio del vettore
        	let first_three= numbers.iter().take(3);
    
    	for n in first_three {
        	println!("{:?}", n);
    	} // Output: [1, 2, 3]
}
