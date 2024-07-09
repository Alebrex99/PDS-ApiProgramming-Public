trait Animali {
    fn fanno_verso(&self);
}

trait Mammiferi: Animali {
    fn si_cibano(&self);
}

trait Uccelli: Animali {
    fn volano(&self);
}

struct Cani;
struct Crow;

impl Animali for Cani {
    fn fanno_verso(&self) {
        println!("Bau Bau");
    }
}

impl Mammiferi for Cani {
    fn si_cibano(&self) {
        println!("I cuccioli di cani sono allattati.");
    }
}

impl Animali for Crow {
    fn fanno_verso(&self) {
        println!("Cra Cra");
    }
}

impl Uccelli for Crow {
    fn volano(&self) {
        println!("I corvi volano.");
    }
}

fn main() {
    let dog = Cani;
    let corvo = Crow;
    dog.fanno_verso();
    dog.si_cibano();
    corvo.fanno_verso();
    corvo.volano();
}