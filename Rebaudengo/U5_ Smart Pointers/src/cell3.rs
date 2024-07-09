use std::cell::Cell;
struct Contatore {
    conteggio: Cell<u32>,
}
impl Contatore {
    fn new() -> Contatore {
        Contatore {
            conteggio: Cell::new(0),
        }
    }
    fn incrementa(&self) {
        let conteggio_attuale = self.conteggio.get();
        self.conteggio.set(conteggio_attuale + 1);
    }
    fn decrementa(&self) {
        let conteggio_attuale = self.conteggio.get();
        self.conteggio.set(conteggio_attuale - 1);
    }
    fn leggi(&self) -> u32 {
        self.conteggio.get()
    }
}
fn main() {
    let contatore = Contatore::new();
    contatore.incrementa();
    contatore.incrementa();
    println!("Conteggio: {}", contatore.leggi());
    contatore.decrementa();
    println!("Conteggio: {}", contatore.leggi());
}