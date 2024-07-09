trait Convertibile {
    type Output;

    fn converti(&self) -> Self::Output;
}

struct NumeroIntero {
    valore: i32,
}
impl Convertibile for NumeroIntero {
    type Output = f64;

    fn converti(&self) -> Self::Output {
        self.valore as f64
    }
}
struct NumeroReale {
    valore: f64,
}
impl Convertibile for NumeroReale {
    type Output = i32;

    fn converti(&self) -> Self::Output {
        self.valore as i32
    }
}

fn main() {
    let numero = NumeroIntero { valore: 42 };
    let valore_convertito: f64 = numero.converti();
    println!("Valore convertito: {}", valore_convertito);

    let numero = NumeroReale { valore: 42.3 };
    let valore_convertito: i32 = numero.converti();
    println!("Valore convertito: {}", valore_convertito);

}