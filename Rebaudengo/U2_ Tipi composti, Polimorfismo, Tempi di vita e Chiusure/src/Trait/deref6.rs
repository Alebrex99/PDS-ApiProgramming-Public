fn main() {
    let mut my_string = String::from("Hello");

    // Utilizzo del dereferenziamento mutevole per modificare la stringa
    let my_string_ref: &mut String = &mut my_string;
    (*my_string_ref).push_str(", world!");

    println!("{}", my_string_ref);
}