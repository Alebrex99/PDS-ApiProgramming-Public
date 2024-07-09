trait Calcolabile {
    fn calcola(&self) -> f64;
}

struct Cerchio {
    raggio: f64,
}

struct Quadrato {
    lato: f64,
}

impl Calcolabile for Cerchio {
    fn calcola(&self) -> f64 {
        std::f64::consts::PI * self.raggio * self.raggio
    }
}

impl Calcolabile for Quadrato {
    fn calcola(&self) -> f64 {
        self.lato * self.lato
    }
}

fn calcola_area<T: Calcolabile>(forma: T) -> f64 {
    forma.calcola()
}

fn main() {
    let cerchio = Cerchio { raggio: 5.0 };
    let quadrato = Quadrato { lato: 4.0 };

    println!("Area del cerchio: {}", calcola_area(cerchio));
    println!("Area del quadrato: {}", calcola_area(quadrato));
}