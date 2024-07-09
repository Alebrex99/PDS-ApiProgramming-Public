use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: i32,
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        other.age.cmp(&self.age)
    }
}

fn main() {
    let john = Person { name: String::from("John"), age: 25 };
    let emma = Person { name: String::from("Emma"), age: 23 };
    let peter = Person { name: String::from("Peter"), age: 25 };

    println!("John is {:?}, compared to Emma.", john.cmp(&emma)); // Stampa: John is Greater, compared to Emma.
    println!("John is {:?}, compared to Peter.", john.cmp(&peter)); // Stampa: John is Equal, compared to Peter.

}