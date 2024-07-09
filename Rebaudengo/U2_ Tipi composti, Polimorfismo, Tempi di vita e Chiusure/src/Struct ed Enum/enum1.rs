fn main() {
    // define enum with multiple variants and data types
    #[derive(Debug)]
    enum Game {
        Quit,
        Print(String),
        Position { x: i32, y: i32 },
        ChangeBackground(i32, i32, i32),
    }

    // initialize enum with values
    let quit = Game::Quit;
    let print = Game::Print(String::from("Hello World!"));
    let position = Game::Position { x: 10, y: 20 };
    let color = Game::ChangeBackground(200, 255, 255);

    // print enum values
    println!("quit = {:?}", quit);
    println!("print = {:?}", print);
    println!("position = {:?}", position);
    println!("color = {:?}", color);
}
