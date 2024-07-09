struct Person {
    name: String,
    age: u32,
}
struct Employee {
    name: String,
    position: String,
}
impl From<Person> for Employee {
    fn from(person: Person) -> Self {
        Employee {
            name: person.name,
            position: String::from("Sviluppatore"), 
        }
    }
}
fn main() {
    let person = Person {
        name: String::from("Mario Rossi"),
        age: 30,
    };
    let employee: Employee = Employee::from(person);
    println!("Nome: {}", employee.name);
    println!("Posizione: {}", employee.position);
}
