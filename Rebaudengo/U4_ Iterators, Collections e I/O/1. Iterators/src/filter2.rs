fn main() {
    let nomi = vec!["Alice", "Bob", "Anna", "Carl", "David"];
    let nomi_filtrati = nomi.iter()
        .filter(|&nome| nome.starts_with("A"));

    for n in nomi_filtrati {
    println!("{:?}", n); // Stampa: ["Alice", "Anna"]
    }
}
