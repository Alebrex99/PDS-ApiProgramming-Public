use std::num::ParseIntError;
use anyhow::Context;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io;

fn sum_file(path: &Path) -> anyhow::Result<i32> {
    let mut file = File::open(path).with_context(|| format!("Missing path {:?}", path)) ? ;
    let mut contents = String::new();
    file.read_to_string(&mut contents).context("File read error") ? ;
    let mut sum = 0;
    for line in contents.lines() {
        sum += line.parse::<i32>().with_context(|| format!("Not a number: {}", line)) ? ;
    }
    Ok(sum)
}
fn handle_sum_file_errors(path: &Path) {
    match sum_file(path) {
        Ok(sum) => println!("sum is {}", sum),
        Err(err) => {
            if let Some(e) = err.downcast_ref::<io::Error>() { println!("{}", err);}  //tratto io::Error
            else if let Some(e) = err.downcast_ref::<ParseIntError>() {println!("{}", err);}  //tratto ParseIntError
            else { unreachable!(); }  //non può capitare
        }
    }
}
fn main() {
    let path = Path::new("file.txt");
    handle_sum_file_errors(&path);
}