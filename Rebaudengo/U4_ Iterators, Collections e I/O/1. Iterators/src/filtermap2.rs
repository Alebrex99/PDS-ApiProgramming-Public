fn main() {
    let dati = vec!["42", "93", "NaN", "42", "18", "77", "invalido"];
    let numeri_validi = dati.into_iter()
        .filter_map(|s| s.parse::<i32>().ok());

    for n in numeri_validi {
        println!("{:?}", n);
    } // Stampa: [42, 93, 18, 77]
}
