fn trova_primo(s: &str, target: char) -> Option<&str> {
    for (i, c) in s.chars().enumerate() {
        if c == target {
            return Some(&s[..i]);
        }
    }
    None
}

fn main() {
    let s = String::from("hello");
    let result;
    {
        let r = trova_primo(&s, 'l');
        result = r.unwrap();
    }
    println!("{}", result); // Stampa "he", la porzione di stringa fino al primo 'l'
}