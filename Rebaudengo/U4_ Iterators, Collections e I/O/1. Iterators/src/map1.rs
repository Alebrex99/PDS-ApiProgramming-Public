fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Creiamo un iteratore dal vettore
    // applichiamo il metodo map per raddoppiare ogni elemento
    
    let doubled_iter = numbers.iter().map(|&x| x * 2);

    // Stampiamo i risultati
    for doubled in doubled_iter {
        println!("{}", doubled);
    }
}



