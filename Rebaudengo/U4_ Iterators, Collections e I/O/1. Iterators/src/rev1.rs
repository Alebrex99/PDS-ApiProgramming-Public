fn main() {
        let numbers = vec![1, 2, 3, 4, 5];
        let reversed_numbers= numbers.iter().rev();

        println!("Numeri originali: {:?}", numbers);

        for n in reversed_numbers {
                println!("Numeri invertiti: {:?}", n);
        }
}

