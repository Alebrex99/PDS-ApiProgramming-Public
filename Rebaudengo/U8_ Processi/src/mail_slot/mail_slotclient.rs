use mail_slot::{MailslotName, MailslotClient};

fn main() {
    // Crea un nome per il mailslot (puoi cambiarlo a tuo piacimento)
    let name = MailslotName::local("naive");

    // Crea un server mailslot
    let mut client = MailslotClient::new(&name).unwrap();

    // Data to be sent
    let numbers = [42, 256, 1024];
    let text = "Hello, server!";

    // Create the message
    let message = format!("{},{},{},{}", numbers[0], numbers[1], numbers[2], text);

    // Write the message to the mail slot
    client.send_message(message.as_bytes()).unwrap();

    client.send_message(b"STOP").unwrap();
}