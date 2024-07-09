use std::io;

fn main() {
    let mut s = String::new();

    if io::stdin().read_line(&mut s).is_ok() {
        println!("Got {}", s.trim() );
    } else {
        println!("Failed to read line!");
        }

    io::stdin().read_line(&mut s).unwrap();
    println!("Got {}", s.trim() );

}