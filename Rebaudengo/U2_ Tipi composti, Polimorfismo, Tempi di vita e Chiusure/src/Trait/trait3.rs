trait Inizializzabile {
    fn inizializza() -> Self;
}

struct Punto {
    x: i32,
    y: i32,
}

impl Inizializzabile for Punto {
    fn inizializza() -> Self {
        Punto { x: 0, y: 0 }
    }
}

fn main() {
    let punto = Punto::inizializza();
    println!("Punto inizializzato: ({}, {})", punto.x, punto.y);
}