// Definiamo una struttura Centimeters
struct Centimeters(f64);

// Definiamo una struttura Inches
struct Inches(f64);

// Implementiamo il tratto From per convertire da Inches a Centimeters
impl From<Inches> for Centimeters {
    fn from(inches: Inches) -> Self {
        Centimeters(inches.0 * 2.54)
    }
}

fn main() {
    let inches = Inches(10.0);
    let centimeters: Centimeters = Centimeters::from(inches);
    println!("10 pollici sono equivalenti a {:.2} centimetri.", centimeters.0);
}