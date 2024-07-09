use std::fs::File;
use std::io;
use std::io::{Write, BufReader, BufRead};


fn main() -> io::Result<()> {
    let path = "myfile";

    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFun")?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}