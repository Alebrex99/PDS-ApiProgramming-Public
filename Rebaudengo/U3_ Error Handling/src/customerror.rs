use std::fs::File;
use std::path::Path;
use std::fmt;
use std::error;
use std::num::ParseIntError;
use std::io::Read;
use std::io;
#[derive(Debug)]
enum SumFileError {
    Io(io::Error),
    Parse(ParseIntError),
}

impl From<io::Error> for SumFileError {
    fn from(err: io::Error) -> Self { SumFileError::Io(err) }
}
impl From<ParseIntError> for SumFileError {
    fn from(err: ParseIntError) -> Self { SumFileError::Parse(err) }
}
impl fmt::Display for SumFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SumFileError::Io(err) => write!(f, "IO error: {}", err),
            SumFileError::Parse(err) => write!(f, "Parse error: {}", err),
        }
    }
}
impl error::Error for SumFileError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            SumFileError::Io(err) => err,
            SumFileError::Parse(err) => err,
        })
    }
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