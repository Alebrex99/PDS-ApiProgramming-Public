use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
struct Point {x: i32, y: i32}

use mail_slot::{MailslotName, MailslotClient};

fn main() {
    let name = MailslotName::local("naive");

    let mut client = MailslotClient::new(&name).unwrap();

    let point1 = Point { x: 1, y: 2 };
    let point2 = Point { x: 2, y: 1 };


    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point1).unwrap();
    println!("serialized = {}", serialized); // Prints serialized = {"x":1,"y":2}

    // Write the message to the mail slot
    client.send_message(serialized.as_bytes()).unwrap();

    let serialized = serde_json::to_string(&point2).unwrap();
    println!("serialized = {}", serialized); // Prints serialized = {"x":2,"y":1}

    // Write the message to the mail slot
    client.send_message(serialized.as_bytes()).unwrap();

    client.send_message(b"STOP").unwrap();
}

