fn find_number(n: i32) -> i32 {
    let mut count = 0;
    let mut sum = 0;
    loop {
        count += 1;
        if count % 5 == 0 { continue; }   	// ignora i multipli di 5
        sum += if count % 3 == 0 { 1 } else { 0 }; // conta i multipli di 3
        if sum == n { break; }			// fermati al n° multiplo di 3
                                               // ma non multiplo di 5
    }
    count					// restituisce il valore trovato
}

fn main() {
    println!("{}", find_number(5) );		// invocazione della funzione
}