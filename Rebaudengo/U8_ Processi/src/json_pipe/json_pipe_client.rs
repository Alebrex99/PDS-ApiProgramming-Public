use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
struct Point {x: i32, y: i32}

fn main() {
    
    let point1 = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point1).unwrap();
    println!("{}", serialized); // Prints serialized = {"x":1,"y":2}

}

