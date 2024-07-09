fn main() {
    let numbers = vec![5, 10, 15, 20, 22, 30];

    // Prende numeri finché sono multipli di 5
    let multiples_of_five = numbers.iter()
        .take_while(|&num| *num % 5 == 0);

    for n in multiples_of_five {
        println!("Multipli di 5: {:?}", n);
    }
}