fn main() {
    let values = [1, 3, 6, 7];

    match &values[..] {	// crea una slice con tutti gli elementi
        // Contiene almeno un elemento, il primo valore è 0
        &[0, ..] => println!("Comincia con 0"),
        // Contiene almeno un elemento, l’ultimo valore è compreso tra 3 e 5
        &[.., v @ 3..=5] => println!("Finisce con {}", v),
        // Contiene almeno due elementi
        &[_, v, ..] => println!("Il secondo valore è {}", v),
        // Contiene un solo elemento
        &[v] => println!("Ha un solo elemento: {}", v),
        // Non contiene elementi
        &[] => println!("E' vuoto")
    }
}