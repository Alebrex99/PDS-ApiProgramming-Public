//mod lib
//use lib::capitalize
use capitalize::capitalize;
use std::io::{stdin, stdout, Write};


//LIBRERIA CLAP
use clap::Parser;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct Args {
    /// Name of the person to greet
    //#[clap(short, long)]
    name: String,
    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}



fn main() {
    //passare la stringa da linea di comando
    let args = Args::parse();
    let mut s_output = String ::new();
    for _ in 0..args.count {
        println!("la parola inserita è {}!", args.name);
        s_output = capitalize(&args.name);
        println!("Stringa modificata: {}", s_output);
    }

    //passare la stringa da standard input
    /*
    let mut s_input = String ::new();
    let mut s_output = String ::new();
    print!(">>>");
    stdout().flush().unwrap();
    stdin().read_line(&mut s_input).unwrap();
    print!("la parola inserita è {} ", s_input);

    s_output = capitalize(&s_input);
    println!("Stringa modificata: {}", s_output);
    */

}
