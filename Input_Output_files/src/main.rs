use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize,Debug)]
struct Test{
    alfa: i32,
    beta: String
}
impl Test{
    fn new(alfa: i32, beta:String) -> Self{
        Self {alfa, beta}
    }
}

fn main() -> serde_json::Result<()> {
    let t = Test::new(32, "hello".to_string());
    let t1 = Test{alfa: 2, beta: "ciao".to_string()};
    let v = vec![t]; //t consumato
    //let s = serde_json::to_string(&t)?; //qui t è già stato consumato
    let s_di_v = serde_json::to_string(&v)?;
    //println!("{}", s); //qui stampa solo il json
    println!("{}", s_di_v); // qui mette il JSON in una lista
    Ok(()) //ritorna un Result, quindi scrivo OK senza il ";" finale
}


