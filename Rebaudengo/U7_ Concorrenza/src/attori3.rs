use actix::prelude::*;
// Definisci i messaggi che l'attore può ricevere
#[derive(Message)]
#[rtype(result = "String")]
struct Saluta(String);
#[derive(Message)]
#[rtype(result = "i32")]
struct Conteggio(i32);
// Definisci l'attore
struct Greeter;
impl Actor for Greeter {
    type Context = Context<Self>;
}
impl Handler<Saluta> for Greeter {
    type Result = String;
    fn handle(&mut self, msg: Saluta, _: &mut Self::Context) -> Self::Result {
        format!("Ciao, {}!", msg.0)
    }
}
impl Handler<Conteggio> for Greeter {
    type Result = i32;
    fn handle(&mut self, msg: Conteggio, _: &mut Self::Context) -> Self::Result {
        //println!("Ho ricevuo {} e restituisco {}!", msg.0, msg.0+1);
        msg.0 + 1
    }
}
#[actix_rt::main]
async fn main() {
    let mut my_num= i32::default();
    // Crea un attore greeter
    let greeter = Greeter.start();

    // Invia un messaggio all'attore
    let result = greeter.send(Saluta("Mondo".to_string())).await;

    match result {
        Ok(response) => println!("Risposta: {}", response),
        Err(e) => println!("Errore: {}", e),
    }
    let result = greeter.send(Conteggio(42)).await;
    match result {
        Ok(response) =>  my_num = response,
        Err(e) => println!("Errore: {}", e),
    }
    println!("Dato ricevuto e elaborato {}", my_num);
}

