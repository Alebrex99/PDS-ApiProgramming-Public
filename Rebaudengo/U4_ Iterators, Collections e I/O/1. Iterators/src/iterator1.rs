// Definiamo una struttura che implementerà il tratto Iterator
struct Contatore {
    count: usize,
    max: usize,
}
// Implementiamo il tratto Iterator per la nostra struttura
impl Iterator for Contatore {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            let val = self.count;
            self.count += 1;
            Some(val)
        } else {
            None
        }
    }
}
fn main() {
    let mut contatore = Contatore { count: 0, max: 10 };

    // Usiamo l'iteratore per stampare i valori
    while let Some(i) = contatore.next() {
        println!("{}", i);
    }
}
