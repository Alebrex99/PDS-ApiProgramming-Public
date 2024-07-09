use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
struct Point {x: i32, y: i32}
#[derive(Serialize, Deserialize, Debug)]
struct X(i32, i32);
#[derive(Serialize, Deserialize, Debug)]
struct Y(i32);
#[derive(Serialize, Deserialize, Debug)]
struct Z;

fn main() {
    let point = Point { x: 1, y: 2 };
    let x = X(0, 0);
    let y = Y(0);
    let z = Z;

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized); // Prints serialized = {"x":1,"y":2}
    // Convert the JSON string back to a Point.
    let deserialized: Point= serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized); // Prints deserialized = Point { x: 1, y: 2 }

    let serialized = serde_json::to_string(&x).unwrap();
    println!("serialized = {}", serialized); // Prints serialized = [0,0]
    let deserialized: X = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized); // Prints deserialized = X(0, 0)

    let serialized = serde_json::to_string(&y).unwrap();
    println!("serialized = {}", serialized); // Prints serialized = 0
    let deserialized: Y = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized); // Prints deserialized = Y(0)

    let serialized = serde_json::to_string(&z).unwrap();
    println!("serialized = {}", serialized); // Prints serialized = null
    let deserialized: Z = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized); // Prints deserialized = Z
}
