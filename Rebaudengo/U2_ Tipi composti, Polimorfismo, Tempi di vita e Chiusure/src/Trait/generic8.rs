trait Suona { fn suona(&self);}
struct Chitarra;
struct Pianoforte;
impl Suona for Chitarra {
    fn suona(&self) {
        println!("Chitarra: Strum!");
    }
}
impl Suona for Pianoforte {
    fn suona(&self) {
        println!("Pianoforte: Ding!");
    }
}
fn esegui_melodia<T: Suona>(strumento: & T) { strumento.suona(); }
fn main() {
    let chitarra = Chitarra;
    let pianoforte = Pianoforte;
    esegui_melodia(&chitarra);
    esegui_melodia(&pianoforte);
}
