use mail_slot::{MailslotServer, MailslotName};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {x: i32, y: i32}

fn main() {
    let name = MailslotName::local("naive");

    // Crea un server mailslot
    let mut server = MailslotServer::new(&name).unwrap();

    println!("Server is running, waiting for messages...");

    'outer: loop {
        while let Some(msg) = server.get_next_unread().unwrap() {
            let msg_str = String::from_utf8(msg).unwrap();

            if msg_str == "STOP" {
                println!("Stop from client");
                break 'outer;
            }
            let deserialized: Point= serde_json::from_str(&msg_str).unwrap();
            println!("message from client {}", msg_str);
            println!("deserialized = {:?}", deserialized);
        }
    }
}
