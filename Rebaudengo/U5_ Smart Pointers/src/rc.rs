use std::rc::Rc;
fn main() {
    // Creiamo un valore Rc contenente una stringa.
    let rc_esempio = Rc::new("Esempio Rc".to_string());
    {   // Cloniamo rc_esempio per creare un nuovo puntatore allo stesso valore.
        let rc_a = Rc::clone(&rc_esempio);
        println!("Conteggio referenze di rc_a: {}", Rc::strong_count(&rc_a));
        {   // Cloniamo ancora per creare un altro puntatore.
            let rc_b = Rc::clone(&rc_a);
            println!("Conteggio referenze di rc_b: {}", Rc::strong_count(&rc_b));
            println!("Conteggio referenze di rc_a: {}", Rc::strong_count(&rc_a));

            // I due Rc sono uguali se i loro valori interni sono uguali.
            println!("rc_a e rc_b sono uguali: {}", rc_a.eq(&rc_b));
            
            // Possiamo usare direttamente i metodi del valore interno.
            println!("Lunghezza del valore dentro rc_a: {}", rc_a.len());
        }
        // Quando rc_b esce dallo scope, il conteggio delle referenze diminuisce.
        println!("Conteggio referenze di rc_a dopo che rc_b è uscito: {}", Rc::strong_count(&rc_a));
    }
    println!("Conteggio referenze di rc_esempio dopo che rc_a è uscito: {}", Rc::strong_count(&rc_esempio));
    // Quando rc_esempio esce dallo scope, rc_esempio e il valore vengono eliminati.
}


