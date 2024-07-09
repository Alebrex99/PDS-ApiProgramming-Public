use std::cell::Cell;
#[derive(Debug)]
struct SomeStruct {
    a: u8,
    b: Cell<u8>,
}
fn main() {

    let my_struct = SomeStruct {
        a: 0,
        b: Cell::new(1),
    };

    // my_struct.a = 100;
    // ERRORE: `my_struct` è immutabile

    my_struct.b.set(100);

    // OK: anche se `my_struct` è immutabile, `b` è una Cell e può essere modificata
    println!("{:?}", my_struct);

}