fn stampa<'a, 'b> (str1: &'a str, str2: &'b str) -> &'a str {
    println!("{}",str2);
    str1

}

fn main() {
    let s1 = String::from("Viva i lifetimes");
    let s2 = String::from("Questa è una stringa di benvenuto");

    let risultato;
    risultato = stampa(&s1, &s2);

    println!("{}", risultato);
}
