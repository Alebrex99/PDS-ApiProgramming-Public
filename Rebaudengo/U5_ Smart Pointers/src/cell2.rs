use std::cell::Cell;
#[derive(Debug)]
struct SomeStruct {
    a: u8,       // il campo a è soggetto a tutti i vincoli del borrow checker
                               
    b: Cell<u8>, // il campo b è soggetto ad un rilassamento di questi vincoli
}
fn main() {

    let my_struct = SomeStruct {
        a: 0,
        b: Cell::new(1),
    };


    let copy_struct = &my_struct;
    my_struct.b.set(100);

    // OK: anche se esiste un Reference, `b` è una Cell e può essere modificata
    
    println!("{:?}", my_struct);
    println!("{:?}", copy_struct);
}
