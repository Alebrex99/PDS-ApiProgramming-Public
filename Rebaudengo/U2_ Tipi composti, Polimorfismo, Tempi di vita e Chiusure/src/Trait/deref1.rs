fn main() {
    let mut s: String = "Hello, World!".to_string();
    let mut s_ref: &str = &s; // Deref coercion: &String -> &str

    println!("{}", s_ref); // Prints: Hello, World!
}