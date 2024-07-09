use std::ops::Deref;

// Definiamo una struttura dati interna
struct MyData {
    value: i32,
}

// Definiamo una struttura esterna che contiene un MyData all'interno di un Box
struct Container {
    data: Box<MyData>,
}

// Implementiamo il tratto Deref per Container
impl Deref for Container {
    type Target = MyData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

fn main() {
    // Creiamo un'istanza di MyData
    let my_data = MyData { value: 42 };

    // Mettiamo MyData dentro un Box e poi lo mettiamo dentro un Container
    let container = Container { data: Box::new(my_data) };

    // Utilizzando dereferenziazione automatica per accedere al valore all'interno di Container
    println!("Value inside Container: {}", container.value);
}