fn consume_closure<F>(f: F)
    where
        F: FnOnce() -> String,
{
    println!("La closure dice: {}", f());
}

fn main() {
    let text = "Hello, world!".to_string();

    let printer = || text; 

    consume_closure(printer);

    // La seguente linea sarebbe errore se decommentata, 
    // perché printer non può essere usata più di una volta.
    // consume_closure(printer);

}
