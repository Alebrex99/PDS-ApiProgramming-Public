struct Contenitore<'a> {
    dati_esteri: &'a str,
    dati_interni: String,
}

fn main() {
    let esterno = String::from("dati esterni");
    let interno = String::from("dati interni");
    let cont = Contenitore { dati_esteri: &esterno, dati_interni: interno };
    println!("Dati esterni: {}, Dati interni: {}", cont.dati_esteri, cont.dati_interni);
}