
use std::fs::File;
use std::io;
use std::io::Read;

fn read_file(name: &str)-> Result<String,io::Error> {
    let r1 = File::open(name);
    let mut file = match r1 {
        Err(why) => return Err(why),
        Ok(file) => file,
    };
    let mut s = String::new();
    let r2 = file.read_to_string(&mut s);
    match (r2) {
        Err(why) => Err(why),
        Ok(_) => Ok(s),
    }
}

fn main ()
{
    let s = read_file("prova.txt");
    let myfile= s.unwrap();

    println!("{}", myfile);
}