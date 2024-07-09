#[derive(Debug)]
struct Player {
    name: String,  // nickname
    health: i32,   // stato di salute (in punti vita)
    level: u8,     // livello corrente
}

impl Player {
    fn with_name(s: String) -> Self {
        Self {name: s, health: 25, level: 1}
        }
    }
fn main() {
    let p1 = Player::with_name("Mario".to_string());

    println!("{:?}", p1);
}
