use std::env::args;
fn main() {
    let args: Vec<String> = args().skip(1).collect();
    if args.len() > 0 {
        // we have args!
        println!("{}", args[0]);
    }
}
