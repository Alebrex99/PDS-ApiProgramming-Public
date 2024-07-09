#[derive(Default)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // Utilizzando il valore predefinito fornito dal tratto Default
    let default_person: Person = Default::default();
    println!("Name: {}", default_person.name); // Stampa: Name:
    println!("Age: {}", default_person.age);  // Stampa: Age: 0
}