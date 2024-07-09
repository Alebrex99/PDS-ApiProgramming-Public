fn main() {
    let name1 = "Matteo";
    let name2 = "Giovanni".to_string();

    greet(name1);
    greet(&name2);
}

fn greet(name: &str) {
        println!("Hello, {}!", name);
}
