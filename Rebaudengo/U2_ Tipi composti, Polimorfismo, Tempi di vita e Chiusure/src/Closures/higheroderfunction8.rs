fn crea_contatore() -> impl FnMut() -> i32 {
    let mut contatore = 0;
    move || {
        contatore += 1;
        contatore
    }
}

fn main() {
    let mut incrementa = crea_contatore();
    println!("Il primo valore è: {}", incrementa());
    println!("Il secondo valore è: {}", incrementa());
}
