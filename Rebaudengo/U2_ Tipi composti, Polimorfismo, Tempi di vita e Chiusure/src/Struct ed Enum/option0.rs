fn guarda_dentro(var: Option<String>)
{  match var {
    None => println!("Non c'e' nulla"),
    Some(mystr) => println!("Stampo {}", mystr),
}
}
fn main() {
    let example1 = Some("Ciao".to_string());
    let example2 = None;

    guarda_dentro(example1);
    guarda_dentro(example2);
}