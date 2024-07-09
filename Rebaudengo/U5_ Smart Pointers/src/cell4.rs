use std::cell::Cell;

fn main() {
    // Creiamo una Cell contenente un valore iniziale.
    let cell = Cell::new(42);

    // Utilizziamo il metodo take per estrarre il valore dalla Cell.
    let taken_value = cell.take();
    println!("Valore estratto dalla Cell: {}", taken_value);

    // Utilizziamo il metodo replace per sostituire il valore all'interno della Cell 
    // e ottenere quello precedente.
    
    let previous_value = cell.replace(99);
    println!("Valore precedente nella Cell: {}", previous_value);

    // Utilizziamo il metodo into_inner per estrarre il valore contenuto nella Cell.
    let inner_value = cell.into_inner();
    println!("Valore estratto dalla Cell con into_inner: {}", inner_value);

    // la cella è stata mossa e dunque non è più accessibile
    // let taken_value = cell.take();
}
