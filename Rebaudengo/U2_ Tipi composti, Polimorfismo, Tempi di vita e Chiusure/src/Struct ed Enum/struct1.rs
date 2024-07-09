#[derive(Debug)]
struct Player {
    name: String,  // nickname
    health: i32,   // stato di salute (in punti vita)
    level: u8,     // livello corrente
}

fn main() {
    let p1 = Player {name: "Mario".to_string(), health: 25, level: 1} ;
    let p2 = Player {name: "Giovanni".to_string(), .. p1};

    println!("{:?} \n{:?}", p1, p2);
}