use std::borrow::Cow;

fn main() {
    let borrowed_data: Cow<str> = Cow::Borrowed("Salve ");

    let mut owned_data: String = borrowed_data.into_owned();
    owned_data.push_str("Mondo!");

    println!("Owned data: {}", owned_data);
}
