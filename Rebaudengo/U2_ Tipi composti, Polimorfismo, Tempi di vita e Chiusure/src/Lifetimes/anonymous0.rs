struct Persona<'a> {
    nome: &'a str,
}

impl Persona<'_> {
    fn saluta(&self) {
        println!("Ciao, sono {}", self.nome);
    }
}

fn main() {
    let nome = String::from("Mario");
    let persona = Persona { nome: &nome };
    persona.saluta();
}
