trait Inizializzabile {
    fn inizializza() -> Self;
}

#[derive(Default)]
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
    let punto1:Punto = Default::default();
    println!("Punto inizializzato: ({}, {})", punto.x, punto.y);
    println!("Punto inizializzato con Default: ({}, {})", punto1.x, punto1.y);
}