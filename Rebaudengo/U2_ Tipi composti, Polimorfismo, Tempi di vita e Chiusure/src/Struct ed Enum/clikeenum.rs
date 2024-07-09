// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
enum Color {
    Red ,
    Green = 50,
    Yellow
}

fn main() {
    println!("Colore Rosso {:?} {:?}", Color::Red, Color::Red as i32);
    println!("Colore Giallo {:?} {:?}", Color::Yellow, Color::Yellow as i32);
}