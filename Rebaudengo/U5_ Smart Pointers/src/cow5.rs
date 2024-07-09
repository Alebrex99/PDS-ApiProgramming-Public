use std::borrow::Cow;

fn main() {
    // Creazione di una stringa owned
    let owned_string = "Hello, World!".to_string();
    let cow_from_owned = Cow::from(owned_string);
    cow_from_owned.to_uppercase();
    println!("{:?}", cow_from_owned); // Output: Cow::Owned("Hello, World!")

    // Creazione di una stringa borrowed
    let borrowed_string = "Rust is awesome!";
    let cow_from_borrowed = Cow::from(borrowed_string);
    println!("{:?}", cow_from_borrowed); // Output: Cow::Borrowed("Rust is awesome!")

    // Creazione di una stringa owned da una stringa borrowed
    let another_borrowed_string = "Strings everywhere!";
    let cow_from_owned_borrowed= Cow::from(another_borrowed_string.to_string());
    println!("{:?}", cow_from_owned_borrowed); // Output: Cow::Owned("Strings everywhere!")
}
