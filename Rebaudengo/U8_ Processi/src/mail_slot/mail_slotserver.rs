use mail_slot::{MailslotServer, MailslotName};

fn main() {
    // Crea un nome per il mailslot (puoi cambiarlo a tuo piacimento)
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
            println!("message from client {}", msg_str);

        }
    }
}
