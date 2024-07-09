use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
enum E {
    W { a: i32, b: i32 },
    X(i32, i32),
    Y(i32),
    Z,
}

fn main() {
    let w = E::W { a: 0, b: 0 }; // Rappresentato come l'oggetto `{"W":{"a":0,"b":0}}`
    let x = E::X(0, 0);          // Rappresentato come l'oggetto `{"X":[0,0]}`
    let y = E::Y(0);             // Rappresentato come l'oggetto `{"Y":0}`
    let z = E::Z;                // Rappresentato come la stringa `"Z"`

    let serialized = serde_json::to_string(&w).unwrap();
    println!("serialized = {}", serialized); // Prints serialized = {"W":{"a":0,"b":0}}
    let deserialized: E = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized); // Prints deserialized = W { a: 0, b: 0 }

    let serialized = serde_json::to_string(&x).unwrap();
    println!("serialized = {}", serialized); // Prints serialized = {"X":[0,0]}
    let deserialized: E = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized); // Prints ddeserialized = X(0, 0)
    
    let serialized = serde_json::to_string(&y).unwrap();
    println!("serialized = {}", serialized); // Prints serialized = {"Y":0}
    let deserialized: E = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized); // Prints deserialized = Y(0)

    let serialized = serde_json::to_string(&z).unwrap();
    println!("serialized = {}", serialized); // Prints serialized = "Z"
    let deserialized: E = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized); // Prints deserialized = Z
}

