fn get_first_word(sentence: &str) -> Option<&str> {
    let first_space = sentence.find(' ')? ;
    Some(&sentence[..first_space])
}

fn main() {
    let sentence = "Hello,world!";

    match get_first_word(sentence) {
        Some(word) => println!("Il primo parola è: {}", word),
        None => println!("La stringa è vuota o non contiene spazi!"),
    }
}
