trait Moltiplicabile {
    fn moltiplica(&self, factor: i32) -> i32;
    fn moltiplica_per_due(&self) -> i32 {
        self.moltiplica(2)
    }
}
struct Numero { valore: i32 }

impl Moltiplicabile for Numero {
    fn moltiplica(&self, factor: i32) -> i32 {
        self.valore * factor
    }

    fn moltiplica_per_due(&self) -> i32 {
        println!("Moltiplicazione per 2");
        self.moltiplica(2) // Ridefinizione del metodo con un comportamento diverso
    }
}

fn main() {
    let numero = Numero { valore: 5 };
    println!("Risultato: {}", numero.moltiplica_per_due());
}
