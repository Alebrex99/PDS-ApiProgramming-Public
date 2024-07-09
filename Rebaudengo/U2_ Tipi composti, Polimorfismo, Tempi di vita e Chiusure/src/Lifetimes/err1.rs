fn confronta(str1: &str, str2: &str) -> &str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let risultato;
    risultato = confronta(&s1, &s2);

    println!("La stringa più lunga è: {}", risultato);
}