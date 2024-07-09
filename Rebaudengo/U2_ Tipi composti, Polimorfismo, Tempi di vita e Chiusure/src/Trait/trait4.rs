trait Descrivibile {
    fn descrizione() -> &'static str;
}

#[derive(Debug)]
struct Libro {
    titolo: String,
    autore: String,
}

#[derive(Debug)]
struct Disco {
    titolo: String,
    cantante_gruppo: String,
    anno: i32
}


impl Descrivibile for Libro {
    fn descrizione() -> &'static str {
        "=> Questo è un libro"
    }
}
impl Descrivibile for Disco {
    fn descrizione() -> &'static str {
        "=> Questo è un disco"
    }
}

fn main() {
    let libro = Libro {
        titolo: String::from("Il Signore degli Anelli"),
        autore: String::from("J.R.R. Tolkien"),
    };
    let disco = Disco {
        titolo: String::from("Out of Time"),
        cantante_gruppo: String::from("R.E.M."),
        anno: 1991
    };
    println!("{:?} {}", libro, Libro::descrizione());
    println!("{:?} {}", disco, Disco::descrizione());
}