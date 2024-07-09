fn genera_contatore() -> impl Fn(i32) -> i32 {
    let contatore = 0;
    move |incremento: i32| contatore + incremento
}

fn main() {
    let conto = genera_contatore();
    println!("Il risultato è: {}", conto(3));
    println!("Il risultato è: {}", conto(10));
}
