use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Team Blue", 10);
    scores.insert("Team Red", 20);

    // Incrementa il punteggio del team "Team Blue" se esiste,
    // altrimenti inserisci un nuovo punteggio
    scores.entry("Team Blue").and_modify(|score| *score += 5).or_insert(15);

    // Incrementa il punteggio del team "Team Green" se esiste,
    // altrimenti inserisci un nuovo punteggio
    scores.entry("Team Green").and_modify(|score| *score += 5).or_insert(15);

    println!("{:?}", scores);
    // Stampa: {"Team Blue": 15, "Team Red": 20, "Team Green": 15}
}
