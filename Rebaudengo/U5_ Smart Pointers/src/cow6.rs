use std::borrow::Cow;
struct Data {
    content: String,
}
fn modify_data(data: &mut Data) -> Cow<str> {
    // Manipoliamo direttamente i dati senza clonarli
    data.content.push_str(", Rust!");
    Cow::from(&data.content)
}
fn main() {
    let mut external_data = Data {
        content: String::from("Hello"),
    };
    // Utilizzando il puntatore, evitiamo il clonaggio dei dati esterni
    let modified_data = modify_data(&mut external_data);
    println!("{}", modified_data); // Stampa "Hello, Rust!"
}

