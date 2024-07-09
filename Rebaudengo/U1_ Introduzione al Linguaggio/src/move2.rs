fn main() {
    let s1 = String::from("hello");

    println!("{s1}");

    let mut s1 = String::from("ciao");

    s1.push_str(" Mamma");

    println!("{s1}");
}