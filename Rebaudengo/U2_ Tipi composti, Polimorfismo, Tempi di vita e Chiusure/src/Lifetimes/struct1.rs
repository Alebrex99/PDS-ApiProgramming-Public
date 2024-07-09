struct Contenitore<'a> {
    dati: &'a str,
}

fn main() {
    let dati = String::from("Dati importanti");
    let cont = Contenitore { dati: &dati };
    println!("{}", cont.dati);
}
