use actix::prelude::*;
// Definisci il messaggio che l'attore può ricevere
#[derive(Message)]
#[rtype(result = "String")]
struct Saluta(String);

// Definisci l'attore
struct Greeter;

impl Actor for Greeter {
    type Context = Context<Self>;
}

// Implementa il gestore del messaggio per l'attore
impl Handler<Saluta> for Greeter {
    type Result = String;

    fn handle(&mut self, msg: Saluta, _: &mut Self::Context) -> Self::Result {
        format!("Ciao, {}!", msg.0)
    }
}
#[actix_rt::main]
async fn main() {
    // Crea un attore greeter
    let greeter = Greeter.start();

    // Invia un messaggio all'attore
    let result = greeter.send(Saluta("Mondo".to_string())).await;

    match result {
        Ok(response) => println!("Risposta: {}", response),
        Err(e) => println!("Errore: {}", e),
    }
}