#[derive(Debug, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let mut people = vec![
        Person { name: String::from("Alice"), age: 30 },
        Person { name: String::from("Bob"), age: 24 },
        Person { name: String::from("Carol"), age: 29 },
    ];

    // Ordina le persone per età usando una chiusura
    people.sort_by(|a, b| a.age.cmp(&b.age));

    println!("{:?}", people);
}
