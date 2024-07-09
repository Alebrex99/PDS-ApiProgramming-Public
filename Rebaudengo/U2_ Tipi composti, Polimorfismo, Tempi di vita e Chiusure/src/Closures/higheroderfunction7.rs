fn crea_messaggio(s: String) -> impl FnOnce() -> String {
    move || s
}

fn main() {
    let messaggio = crea_messaggio("Ciao Rust!".to_string());
    println!("{}", messaggio());
}
