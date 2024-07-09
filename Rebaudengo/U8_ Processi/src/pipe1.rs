

use std::process::Command;
use std::io::prelude::*;
use std::process::Stdio;
static PANGRAM: &'static str = "Stringa di prova organizzata su 2 righe\nQuesta e' la seconda riga\n";
fn main() {
    // Spawn the wc command
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(process) => process,
    };

    // Write a string to the stdin of wc.
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why),
        Ok(_) => println!("sent my string to wc"),
    }
    // Because stdin does not live after the above calls, it is `drop`ed, and the pipe is closed.
    // This is very important, otherwise wc wouldn't start processing the input we just sent.

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}
