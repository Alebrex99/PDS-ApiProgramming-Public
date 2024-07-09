use std::num::ParseIntError;

use std::fs::File;
use std::io;
use std::fs;
use std::error::Error;
use std::io::Read;
use std::path::Path;
use std::error;

fn sum_file(path: &Path) -> Result<i32, Box<dyn error::Error>> {
    let mut file = File::open(path) ? ; 	   // io::Error -> Box<dyn error::Error>
    let mut contents = String::new();
    file.read_to_string(&mut contents) ? ; // io::Error -> Box<dyn error::Error>
    let mut sum = 0;
    for line in contents.lines() {
        sum += line.parse::<i32>() ? ;       // ParseIntError -> Box<dyn error::Error>
    }
    Ok(sum)
}
fn handle_sum_file_errors(path: &Path) {
    match sum_file(path) {
        Ok(sum) => println!("sum is {}", sum),
        Err(err) => {
            if let Some(e) = err.downcast_ref::<io::Error>()
                        {
                            //tratto io::Error
                            println!("Errore di I/O nella gestione del file: {}", e);
                        }
            else if let Some(e) = err.downcast_ref::<ParseIntError>()
            {
                //tratto ParseIntError
                println!("Errore nell'elaborazione del file: {}", e);
            }
            else { unreachable!(); }  //non può capitare
        }
    }
}

fn main() {
    let path = Path::new("file.txt");
    handle_sum_file_errors(&path);
}