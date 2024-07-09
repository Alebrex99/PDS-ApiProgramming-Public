use std::io;
use rand::Rng;

fn main() {

    let mut rng = rand::thread_rng();

    let random_number = rng.gen_range(0..=9);

    // Scrive il numero casuale nello stdout
    println!("{}", random_number);


    loop {
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read response");

        if response != "ACK\n" {
            break;
        }
            
        let random_number = rng.gen_range(0..=9);

        // Scrive il numero casuale nello stdout
        println!("{}", random_number);
    }
}
