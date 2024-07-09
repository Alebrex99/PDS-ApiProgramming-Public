struct Message {
    content: String,
    sender_id: u32,
}

fn main() {
    let msg = Message {
        content: "Ciao dal thread principale!".to_string(),
        sender_id: 42,
    };

    // Crea un nuovo thread e invia il messaggio.
    std::thread::spawn(move || {
        println!("Messaggio ricevuto: {}", msg.content);
    })
        .join()
        .unwrap();
}
