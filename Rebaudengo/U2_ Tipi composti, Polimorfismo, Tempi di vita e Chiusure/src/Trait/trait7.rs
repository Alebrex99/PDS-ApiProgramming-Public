trait Moltiplicabile {
    fn moltiplica(&self, factor: i32) -> i32;
    fn moltiplica_per_due(&self) -> i32 {
        self.moltiplica(2)
    }
}
struct Numero {valore: i32}
struct Numero2 { valore: i32, altro: i32 }
impl Moltiplicabile for Numero {
    fn moltiplica(&self, factor: i32) -> i32 {
        self.valore * factor
    }
    fn moltiplica_per_due(&self) -> i32 {
        println!("Moltiplicazione per 2");
        self.moltiplica(2) // Ridefinizione del metodo con un comportamento diverso
    }
}
impl Moltiplicabile for Numero2 {
    fn moltiplica(&self, factor: i32) -> i32 {
        self.valore * factor
    }
}
fn main() {
    let numero = Numero2 { valore: 5, altro: 1 };
    println!("Risultato: {}", numero.moltiplica_per_due());
}
