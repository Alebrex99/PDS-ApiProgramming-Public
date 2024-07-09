fn main() {
    let mut s = "ciao mamma".to_string();
    s.push_str(" come stai");
    let mut s2 = s.replace("ciao", "Ciao");
    s2.insert(10, ',');

    println!("{}", s2);

    let s = "tschüß".to_string();
    let s2 = s.to_uppercase();

    println!("{}", s2);
}