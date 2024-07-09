use std::rc::Rc;

fn main() {
    let mut valore = Rc::new(5);
    {
        println!("Valore: {:?}", valore);
        let copia = Rc::clone(&valore);
        match Rc::get_mut(&mut valore) {
            Some(v) => *v += 10, // Modifica il valore se è possibile.
            None => println!("Non è possibile ottenere un riferimento mutabile."),
        }
    }
    // Prova a ottenere un riferimento mutabile.
    match Rc::get_mut(&mut valore) {
        Some(v) => *v += 10, // Modifica il valore se è possibile.
        None => println!("Non è possibile ottenere un riferimento mutabile."),
    }

    println!("Valore: {:?}", valore);
}