use::std::io;

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn main() {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let word = first_word(&s);

    println!("the first word is: {}", word);
}