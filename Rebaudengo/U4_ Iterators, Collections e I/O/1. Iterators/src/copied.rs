fn main() {
    let tuple_vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];

    // Otteniamo un iteratore che produce copie delle tuple originali
    let copied_iter = tuple_vec.iter().copied();

    // Stampiamo ogni tupla nel nuovo iteratore
    for tuple in copied_iter {
        println!("Tuple: {:?}", tuple);
    }
    for tuple in tuple_vec{
        println!("Tuple: {:?}", tuple);
    }
}


