fn main() {
        let numbers = vec![1, 3, 5, 7, 2, 7, 8, 9, 10];

        // Salta numeri fino a quando non trova un numero pari
        let skipped = numbers.iter()
            		 .skip_while(|&num| *num % 2 != 0);
    
    for n in skipped {
    println!("Tutti i numeri a partire dal primo pari: {:?}", n);
    }
}
