fn main() {
    let numbers = vec![2, 4, 6, 8, 10];

    // Verifichiamo se tutti gli elementi sono pari
    let all_even = numbers.iter().all(|&x| x % 2 == 0);

    if all_even {
        println!("Tutti gli elementi sono pari.");
    } else {
        println!("Almeno un elemento non è pari.");
    }
}

