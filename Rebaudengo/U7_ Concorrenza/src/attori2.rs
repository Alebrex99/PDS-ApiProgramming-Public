use actix::prelude::*;

// Definisci i messaggi che l'attore può ricevere
#[derive(Message)]
#[rtype(result = "()")]
struct NuovoOrdine {
    cliente: String,
    prodotto: String,
    quantita: u32,
}

struct GestoreOrdini;

impl Actor for GestoreOrdini {
    type Context = Context<Self>;
}

// Implementa il gestore del messaggio per l'attore
impl Handler<NuovoOrdine> for GestoreOrdini {
    type Result = ();

    fn handle(&mut self, msg: NuovoOrdine, _: &mut Self::Context) -> Self::Result {
        println!(
            "Nuovo ordine da {} per {} x {}",
            msg.cliente, msg.prodotto, msg.quantita
        );
    }
}

#[actix_rt::main]
async fn main() {
    // Crea un attore gestore degli ordini
    let gestore_ordini = GestoreOrdini.start();

    // Invia alcuni messaggi all'attore
    gestore_ordini
        .send(NuovoOrdine {
            cliente: "Alice".to_string(),
            prodotto: "Libro".to_string(),
            quantita: 2,
        })
        .await
        .unwrap();

    gestore_ordini
        .send(NuovoOrdine {
            cliente: "Bob".to_string(),
            prodotto: "Computer".to_string(),
            quantita: 1,
        })
        .await
        .unwrap();

}

