use std::num::ParseIntError;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum SumFileError {
    #[error("IO Error {0}")]
    Io(#[from] io::Error),

    #[error("Parse Error {0}")]
    Parse(#[from] ParseIntError),
}





fn sum_file(path: &Path) -> Result<i32, SumFileError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut sum = 0;
    for line in contents.lines() {
        sum += line.parse::<i32>()?;
    }
    Ok(sum)
}

fn handle_sum_file_errors(path: &Path) {
    match sum_file(path) {
        Ok(sum) => println!("the sum is {}", sum),
        Err(SumFileError::Io(err)) => { println!("{}", err)},
        Err(SumFileError::Parse(err)) => { println!("{}", err)},
    }
}

fn main() {
    let path = Path::new("file.txt");
    handle_sum_file_errors(&path);
}