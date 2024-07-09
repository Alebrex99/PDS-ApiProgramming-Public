struct S(u8);

fn f<'a>(x: &S, y: &'a S) -> &'a u8 {
    &y.0
}

fn print_byte(byte: &u8) {
    println!("Byte: {}", byte);
}

fn main() {
    let v1 = S(1);
    let v2 = S(2);

    let r = f(&v1, &v2);

    let v2 = v1;

    print_byte(r);
}