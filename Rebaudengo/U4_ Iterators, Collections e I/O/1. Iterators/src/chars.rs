

fn main() {
    let words = vec!["hello", "world"];
    let chars = "123";

    // Concateniamo l'iteratore delle parole con l'iteratore dei caratteri
    let chained_sequence = words.iter()
        .flat_map(|word| word.chars())
        .chain(chars.chars());

    for n in chained_sequence{
        println!("Carattere: {:?}", n);
    }

}
