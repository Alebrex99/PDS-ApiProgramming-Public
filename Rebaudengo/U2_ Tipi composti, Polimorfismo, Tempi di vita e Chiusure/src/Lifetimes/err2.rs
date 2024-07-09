struct S(u8);

fn f<'a>(x: &S, y: &'a S) -> &'a u8 {
    &y.0
}

fn print_byte(byte: &u8) {
    println!("Byte: {}", byte);
}

fn main() {
    let v1 = S(1);
    let mut v2 = S(2);

    let r = f(&v1, &v2);

    // L'operazione seguente crea un problema di sicurezza!
    v2 = v1;

    // In questo punto, r non è più valido perché v2 ha preso il posto di v1.
    // Questo comportamento potrebbe causare un comportamento indefinito.
    print_byte(r); // ERRORE! Il riferimento non è più valido.
}