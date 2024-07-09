
use std::cell::RefCell;
fn main() {
    let mut c = RefCell::new(5);
    println!("Il dato è {:?}", c);
    {
        *c.get_mut() += 5;

        println!("Il dato aggiornato è {:?}", c);

        let mut m = c.borrow_mut();

        if (c.try_borrow().is_err())
        {
            println!("Non posso fare un altro prestito"); // Si entra qua
        }
        *m = 6;
        println!("Il dato è {:?}", m);
    }
    if (c.try_borrow().is_ok())
    {
        println!("Posso fare un altro prestito");
        let m = c.borrow();
        println!("Il dato è {:?}", m);
        if (c.try_borrow_mut().is_ok())
        {
            println!("Posso fare un altro prestito"); // non si entra qua
            let mut m2 = c.borrow_mut();
        }
        drop(m);
        let val = c.into_inner();
        println!("Valore Finale dopo la distruzione del RefCell{:?}", val);

    }
}