use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Team Blue", 10);
    scores.insert("Team Red", 20);

    // Incrementa il punteggio del team "Team Blue" se esiste,
    // altrimenti inserisci un nuovo punteggio
    scores.entry("Team Blue")
        .and_modify(|score| *score += 5)
        .or_insert(15);

    // Calcola un punteggio predefinito per il team "Team Green" solo se non esiste già,
    // altrimenti utilizza il punteggio esistente
    scores.entry("Team Green")
        .and_modify(|score| *score += 5)
        .or_insert_with(|| calculate_default_score("Team Green"));

    println!("{:?}", scores); // Stampa: {"Team Blue": 15, "Team Red": 20, "Team Green": 25}
}
// Funzione per calcolare il punteggio predefinito per un nuovo team
fn calculate_default_score(team: &str) -> i32 {
// Supponiamo che il punteggio predefinito sia il doppio della lunghezza del nome del team
    (team.len() as i32) * 2
}