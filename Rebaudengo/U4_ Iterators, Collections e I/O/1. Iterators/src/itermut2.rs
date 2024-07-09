fn main() {
    let mut v = vec![String::from("a"), String::from("b"), String::from("c")];

    for s in &mut v {
        s.push_str("1"); // s: &mut String - Modifico il contenuto del vettore
    }

    for s in v.iter_mut() {
        s.push_str("bis");
    }

    for s in &v {
        println!("{}", s); // Output: a1bis, b1bis, c1bis
    }
}
